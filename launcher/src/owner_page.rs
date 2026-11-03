//! The owner gate in the window (`horizon_launcher::account::launch`): the screen that asks for a
//! Microsoft account that owns Minecraft: Java Edition. It covers the launcher once for players
//! without one (those who played offline before the gate), and again whenever Play is refused.
//! The sign-in runs on it, code and all.
use super::*;

impl Launcher {
    /// Whether the screen covers the window: no owner, and either Play was refused or the player
    /// hasn't read it yet. The data question comes first.
    pub(crate) fn asking_owner(&self) -> bool {
        (self.owner_shown() == Some(false) || sign_in_preview())
            && (self.owner_screen_open || !self.settings.owner_notice_seen || sign_in_preview())
            && !self.asking_consent()
            && self.opening.is_none()
    }

    /// `owner` as the window shows it: the developer option that lets offline names play without
    /// an owner (`dev_options`) hides the gate, as if the keychain hadn't answered yet.
    pub(crate) fn owner_shown(&self) -> Option<bool> {
        match self.owner {
            Some(false) if self.settings.dev_offline => None,
            owner => owner,
        }
    }

    /// Opens the screen and starts the sign-in on it.
    pub(crate) fn sign_in_to_play(&mut self) {
        self.owner_screen_open = true;
        self.start_sign_in();
    }

    /// Closes it; it comes back with the next refused Play.
    fn close_owner_screen(&mut self) {
        self.cancel_sign_in();
        self.owner_screen_open = false;
        if !self.settings.owner_notice_seen {
            self.settings.owner_notice_seen = true;
            self.settings.save();
        }
    }

    /// Activates Demo / Offline mode for inspecting the launcher interface without Microsoft login.
    pub(crate) fn start_demo_mode(&mut self) {
        let mut accounts = Accounts::load();
        let demo_name = if accounts.offline.is_empty() {
            "Demo_Oyuncu".to_string()
        } else {
            accounts
                .offline
                .first()
                .cloned()
                .unwrap_or_else(|| "Demo_Oyuncu".to_string())
        };
        let _ = accounts.play_as_with(
            account::store(),
            horizon_launcher::account::Player::Offline(demo_name),
            true,
        );
        let _ = accounts.save();
        self.accounts = accounts;
        self.settings.dev_offline = true;
        self.settings.owner_notice_seen = true;
        self.settings.save();
        self.cancel_sign_in();
        self.owner_screen_open = false;
    }

    pub(crate) fn owner_screen(&mut self, cx: &mut Context<Self>) -> Stateful<Div> {
        let t = self.theme;
        let n = t.night;
        let configured = account::client_id().is_some();
        let state = match &self.sign_in_state {
            _ if sign_in_preview() => Shown::Code("https://www.microsoft.com/link", "K7Q-4MXR"),
            _ if !configured => Shown::Error(tr(account::NOT_CONFIGURED)),
            Some(SignInState::Starting) => Shown::Starting("Asking Microsoft…"),
            Some(SignInState::Working(phase)) => Shown::Starting(phase.label()),
            Some(SignInState::Code(code)) => Shown::Code(&code.verification_uri, &code.user_code),
            Some(SignInState::Done(Err(e))) => Shown::Error(e),
            Some(SignInState::Done(Ok(_))) | None => Shown::Idle,
        };

        let card_box = || {
            div()
                .relative()
                .p(px(14.))
                .rounded(px(theme::night::RADIUS_CONTROL))
                .bg(plate::alpha(n.card_hi, n.card_hi_alpha))
                .border_1()
                .border_color(plate::alpha(n.stroke, n.stroke_alpha))
                .flex()
                .flex_col()
                .gap(px(8.))
        };

        let bullet = |txt: &str| {
            div()
                .flex()
                .items_start()
                .gap(px(6.))
                .child(
                    div()
                        .mt(px(5.))
                        .size(px(4.))
                        .rounded_full()
                        .bg(shell::c(n.ink_soft)),
                )
                .child(
                    shell::text(theme::night::CAPTION, shell::c(n.ink_soft)).child(txt.to_string()),
                )
        };

        // Header
        let header = div()
            .flex()
            .flex_col()
            .gap(px(4.))
            .child(
                shell::text(theme::night::CAPTION, shell::c(n.muted))
                    .font_weight(FontWeight::SEMIBOLD)
                    .child("CREW CLIENT · HESAP SEÇİMİ"),
            )
            .child(
                shell::text(theme::night::TITLE, shell::c(n.ink))
                    .child(tr("Sign in to play")),
            )
            .child(
                shell::text(theme::night::UI, shell::c(n.ink_soft))
                    .child("Resmi lisanslı Microsoft hesabınızla oturum açabilir veya çevrimdışı yerel demo profiliyle devam edebilirsiniz."),
            );

        // Card 1: Microsoft Hesabı
        let mut ms_card = card_box()
            .child(
                div()
                    .flex()
                    .items_center()
                    .justify_between()
                    .child(
                        shell::text(theme::night::UI, shell::c(n.ink))
                            .font_weight(FontWeight::BOLD)
                            .child("1. Microsoft Hesabıyla Giriş Yap"),
                    )
                    .child(
                        div()
                            .px(px(8.))
                            .py(px(2.))
                            .rounded(px(theme::night::RADIUS_CONTROL))
                            .bg(plate::alpha(n.accent, 0.15))
                            .child(
                                shell::text(theme::night::CAPTION, shell::c(n.accent))
                                    .font_weight(FontWeight::SEMIBOLD)
                                    .child("Önerilen · Çevrimiçi"),
                            ),
                    ),
            )
            .child(bullet("Diğer başlatıcılar gibi, CREW Client da Minecraft: Java Edition sahibi bir Microsoft hesabı gerektirir."))
            .child(bullet("Minecraft kurulumu gerekmez: CREW Client hesabınız için dosyaları doğrudan Mojang'dan indirir."))
            .child(bullet("Dünyalarınız, çevrimdışı adlarınız ve ayarlarınız olduğu gibi korunur."))
            .child(bullet("oyna.crewnetwork.space ve tüm çok oyunculu sunuculara tam erişim."))
            .child(bullet("OAuth 2.0 RFC 8628 Device Code güvencesi (Şifreniz asla kaydedilmez)."));

        match state {
            Shown::Code(uri, user_code) => {
                let copy = user_code.to_string();
                let open_uri = uri.to_string();
                ms_card = ms_card.child(
                    div()
                        .mt(px(6.))
                        .p(px(12.))
                        .rounded(px(theme::night::RADIUS_CONTROL))
                        .bg(plate::alpha(n.card, 0.8))
                        .border_1()
                        .border_color(plate::alpha(n.stroke, n.stroke_alpha))
                        .flex()
                        .items_center()
                        .justify_between()
                        .gap(px(12.))
                        .child(
                            div()
                                .flex_1()
                                .min_w_0()
                                .flex()
                                .flex_col()
                                .gap(px(2.))
                                .child(
                                    shell::text(theme::night::CAPTION, shell::c(n.ink_soft)).child(
                                        trf(
                                            "Open {link} in your browser and enter this code:",
                                            &[("link", short_link(uri))],
                                        ),
                                    ),
                                )
                                .child(
                                    div()
                                        .font_family(theme::MONO)
                                        .font_weight(FontWeight::BOLD)
                                        .text_size(px(26.))
                                        .line_height(px(32.))
                                        .text_color(shell::c(n.ink))
                                        .child(user_code.to_string()),
                                ),
                        )
                        .child(
                            div()
                                .flex()
                                .gap(px(6.))
                                .child(
                                    shell::action("owner-open", shell::Action::Main, &t)
                                        .child(shell::label("Bağlantıyı Aç"))
                                        .on_click(move |_, _, cx| cx.open_url(&open_uri)),
                                )
                                .child(
                                    shell::action("owner-copy", shell::Action::Raised, &t)
                                        .child(shell::label(tr("Copy code").to_string()))
                                        .on_click(move |_, _, cx| {
                                            copy_to_clipboard(
                                                cx,
                                                gpui::ClipboardItem::new_string(copy.clone()),
                                            )
                                        }),
                                ),
                        ),
                );
            }
            Shown::Starting(what) => {
                ms_card = ms_card.child(
                    div()
                        .mt(px(4.))
                        .p(px(10.))
                        .rounded(px(theme::night::RADIUS_CONTROL))
                        .bg(plate::alpha(n.card, 0.8))
                        .child(
                            shell::text(theme::night::UI, shell::c(n.ink_soft))
                                .child(tr(what).to_string()),
                        ),
                );
            }
            Shown::Error(e) => {
                let show_config_input = !configured || e.contains("invalid_client");
                if show_config_input {
                    let focused = self.client_id_focused;
                    let input_field = div()
                        .id("client-id-input")
                        .relative()
                        .w_full()
                        .h(px(34.))
                        .px(px(10.))
                        .py(px(4.))
                        .rounded(px(theme::night::RADIUS_CONTROL))
                        .overflow_hidden()
                        .border_2()
                        .border_color(if focused { t.accent } else { t.outline })
                        .bg(t.well)
                        .font_family(theme::MONO)
                        .text_size(px(12.))
                        .cursor_text()
                        .track_focus(&self.client_id_focus)
                        .child(line_edit::content(
                            &self.client_id_input,
                            &t,
                            focused,
                            false,
                            false,
                            "Örn: 00000000-0000-0000-0000-000000000000",
                        ))
                        .on_click(cx.listener(|this, _: &ClickEvent, window, cx| {
                            window.focus(&this.client_id_focus, cx);
                            cx.notify();
                        }))
                        .on_key_down(cx.listener(|this, ev: &gpui::KeyDownEvent, window, cx| {
                            match line_edit::handle(&mut this.client_id_input, ev, cx) {
                                line_edit::Key::Pass => return,
                                line_edit::Key::Escape => window.blur(cx),
                                line_edit::Key::Enter => {
                                    this.save_client_id_and_sign_in();
                                }
                                _ => {}
                            }
                            cx.stop_propagation();
                            cx.notify();
                        }));
                    let input_field =
                        line_edit::mouse(input_field, cx, |this| &mut this.client_id_input);

                    ms_card = ms_card
                        .child(
                            shell::text(theme::night::CAPTION, shell::c(n.danger))
                                .font_weight(FontWeight::BOLD)
                                .child("Azure Uygulama Kimliği (Client ID) Gerekli:"),
                        )
                        .child(
                            shell::text(theme::night::CAPTION, shell::c(n.ink_soft)).child(
                                "1. entra.microsoft.com adresine gidin > Uygulama Kayıtları > Yeni Kayıt\n2. Desteklenen tür: 'Herhangi bir kuruluş dizini ve kişisel Microsoft hesapları'\n3. Yeniden yönlendirme: Mobil/Masaüstü > https://login.microsoftonline.com/common/oauth2/nativeclient\n4. API İzinleri: XboxLive.signin ve offline_access ekleyin\n5. Kimlik Doğrulama: 'Genel istemci akışlarına izin ver' = Evet\n6. Uygulama Kimliği'ni yapıştırın:",
                            ),
                        )
                        .child(input_field)
                        .child(
                            div()
                                .flex()
                                .justify_end()
                                .child(
                                    shell::action("owner-save-signin", shell::Action::Main, &t)
                                        .child(shell::label("Kaydet ve Giriş Yap"))
                                        .on_click(cx.listener(|this, _: &ClickEvent, _, cx| {
                                            this.save_client_id_and_sign_in();
                                            cx.notify();
                                        })),
                                ),
                        );
                } else {
                    let msg = if e == "Sign-in cancelled." {
                        "Giriş kullanıcı tarafından iptal edildi.".to_string()
                    } else if e.contains("expired_token")
                        || e.contains("timeout")
                        || e.contains("timed out")
                    {
                        "Giriş oturumu zaman aşımına uğradı. Lütfen tekrar deneyin.".to_string()
                    } else if e.contains("403") || e.contains("Forbidden") {
                        "Mojang API erişimi reddedildi (403). Azure uygulamasının Mojang onayı ve XboxLive.signin iznine sahip olduğundan emin olun.".to_string()
                    } else {
                        format!("Microsoft kimlik doğrulama hatası: {e}")
                    };
                    ms_card = ms_card
                        .child(shell::text(theme::night::CAPTION, shell::c(n.danger)).child(msg))
                        .child(div().flex().justify_end().child(
                            microsoft::sign_in_button("owner-sign-in-retry", &t).on_click(
                                cx.listener(|this, _: &ClickEvent, _, cx| {
                                    this.sign_in_to_play();
                                    cx.notify();
                                }),
                            ),
                        ));
                }
            }
            Shown::Idle => {
                ms_card = ms_card.child(div().mt(px(4.)).flex().justify_end().child(
                    microsoft::sign_in_button("owner-sign-in", &t).on_click(cx.listener(
                        |this, _: &ClickEvent, _, cx| {
                            this.sign_in_to_play();
                            cx.notify();
                        },
                    )),
                ));
            }
        }

        // Card 2: Yerel Demo Profili
        let demo_card = card_box()
            .child(
                div()
                    .flex()
                    .items_center()
                    .justify_between()
                    .child(
                        shell::text(theme::night::UI, shell::c(n.ink))
                            .font_weight(FontWeight::BOLD)
                            .child("2. Yerel Demo Profili"),
                    )
                    .child(
                        div()
                            .px(px(8.))
                            .py(px(2.))
                            .rounded(px(theme::night::RADIUS_CONTROL))
                            .bg(plate::alpha(n.card, 0.9))
                            .child(
                                shell::text(theme::night::CAPTION, shell::c(n.muted))
                                    .font_weight(FontWeight::SEMIBOLD)
                                    .child("Çevrimdışı Mod"),
                            ),
                    ),
            )
            .child(bullet(
                "Microsoft hesabı veya lisans olmadan başlatıcıyı ve oyunu deneyin.",
            ))
            .child(bullet(
                "Yalnızca yerel tek oyunculu dünyalar için geçerlidir.",
            ))
            .child(bullet(
                "Çevrimiçi çok oyunculu (multiplayer) sunucularda çalışmaz.",
            ))
            .child(bullet(
                "Korsan veya crackli hesap değildir; resmi sunucu doğrulamasını atlamaz.",
            ))
            .child(
                div().mt(px(4.)).flex().justify_end().child(
                    shell::action("owner-demo", shell::Action::Raised, &t)
                        .child(shell::label("Yerel Demo Profili ile Başlat"))
                        .on_click(cx.listener(|this, _: &ClickEvent, _, cx| {
                            this.start_demo_mode();
                            cx.notify();
                        })),
                ),
            );

        let waiting = matches!(state, Shown::Code(..)).then(|| {
            shell::text(theme::night::CAPTION, shell::c(n.muted))
                .relative()
                .child(tr("Waiting for Microsoft… the window goes on by itself.").to_string())
        });

        let footer = div()
            .flex()
            .items_center()
            .justify_between()
            .mt(px(8.))
            .child(div().children(waiting))
            .child(
                shell::action("owner-later", shell::Action::Quiet, &t)
                    .child(shell::label(tr("Not now").to_string()))
                    .on_click(cx.listener(|this, _: &ClickEvent, _, cx| {
                        this.close_owner_screen();
                        cx.notify();
                    })),
            );

        let card = plate::cut(
            theme::night::PIXEL_CORNER,
            2,
            Some(plate::alpha(n.card, 0.98)),
            Some((plate::alpha(n.stroke, 2. * n.stroke_alpha), 1.)),
        )
        .flex()
        .flex_col()
        .gap(px(12.))
        .w(px(580.))
        .max_w_full()
        .p(px(28.))
        .child(header)
        .child(ms_card)
        .child(demo_card)
        .child(footer);

        div()
            .id("owner-gate")
            .absolute()
            .top_0()
            .left_0()
            .size_full()
            .flex()
            .flex_col()
            .items_center()
            .justify_center()
            .px(px(32.))
            .py(px(24.))
            .bg(plate::alpha(n.page, 0.8))
            .on_scroll(|_, _, cx| cx.stop_propagation())
            .occlude()
            .child(card)
    }
}

/// What the sheet shows of the sign-in.
#[derive(Clone, Copy)]
enum Shown<'a> {
    Idle,
    /// What the sign-in is doing.
    Starting(&'static str),
    /// The page and the code to enter there.
    Code(&'a str, &'a str),
    Error(&'a str),
}

/// `HORIZON_LAUNCHER_PAGE=sign-in` opens the sheet with a made-up code, for screenshots: nothing
/// asks Microsoft, and the gate itself is as it was (no account, no Play).
pub(super) fn sign_in_preview() -> bool {
    static PREVIEW: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    *PREVIEW.get_or_init(|| std::env::var("HORIZON_LAUNCHER_PAGE").as_deref() == Ok("sign-in"))
}

/// `https://www.microsoft.com/link` as `microsoft.com/link`.
fn short_link(uri: &str) -> &str {
    let uri = uri
        .trim_start_matches("https://")
        .trim_start_matches("http://");
    uri.strip_prefix("www.").unwrap_or(uri)
}

#[cfg(test)]
mod tests {
    #[test]
    fn links_are_shown_as_typed() {
        assert_eq!(
            super::short_link("https://www.microsoft.com/link"),
            "microsoft.com/link"
        );
        assert_eq!(
            super::short_link("https://login.live.com/oauth20_remoteconnect.srf"),
            "login.live.com/oauth20_remoteconnect.srf"
        );
    }
}
