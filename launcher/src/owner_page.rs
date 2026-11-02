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
            accounts.offline.first().cloned().unwrap_or_else(|| "Demo_Oyuncu".to_string())
        };
        let _ = accounts.play_as_with(account::store(), horizon_launcher::account::Player::Offline(demo_name), true);
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
        // The screenshot preview shows a code without asking Microsoft for one.
        let state = match &self.sign_in_state {
            _ if sign_in_preview() => Shown::Code("https://www.microsoft.com/link", "K7Q-4MXR"),
            _ if !configured => Shown::Error(tr(account::NOT_CONFIGURED)),
            Some(SignInState::Starting) => Shown::Starting("Asking Microsoft…"),
            Some(SignInState::Working(phase)) => Shown::Starting(phase.label()),
            Some(SignInState::Code(code)) => Shown::Code(&code.verification_uri, &code.user_code),
            Some(SignInState::Done(Err(e))) => Shown::Error(e),
            Some(SignInState::Done(Ok(_))) | None => Shown::Idle,
        };
        let line = |s: &str, color: u32| {
            shell::text(theme::night::UI, shell::c(color))
                .relative()
                .child(tr(s).to_string())
        };
        let mut body = div()
            .relative()
            .flex()
            .flex_col()
            .gap(px(10.))
            .child(
                shell::text(theme::night::CAPTION, shell::c(n.muted))
                    .font_weight(FontWeight::SEMIBOLD)
                    .child(tr("Microsoft account").to_string()),
            )
            .child(
                shell::text(theme::night::TITLE, shell::c(n.ink))
                    .mb(px(4.))
                    .child(tr("Sign in to play").to_string()),
            )
            .child(line(
                "Horizon needs a Microsoft account that owns Minecraft: Java Edition, like other launchers.",
                n.ink_soft,
            ))
            .child(line(
                "No Minecraft install needed: Horizon downloads it from Mojang for your account.",
                n.ink_soft,
            ))
            .child(line(
                "Your worlds, offline names and settings stay where they are.",
                n.muted,
            ));
        let raised = || {
            div()
                .relative()
                .mt(px(10.))
                .p(px(18.))
                .rounded(px(theme::night::RADIUS_CONTROL))
                .bg(plate::alpha(n.card_hi, n.card_hi_alpha))
                .border_1()
                .border_color(plate::alpha(n.stroke, n.stroke_alpha))
        };
        body = match state {
            Shown::Code(uri, user_code) => {
                let copy = user_code.to_string();
                body.child(
                    raised()
                        .flex()
                        .items_center()
                        .justify_between()
                        .gap(px(16.))
                        .child(
                            div()
                                .flex_1()
                                .min_w_0()
                                .flex()
                                .flex_col()
                                .gap(px(4.))
                                .child(
                                    shell::text(theme::night::CAPTION, shell::c(n.ink_soft)).child(
                                        trf(
                                            "Open {link} in your browser and enter this code:",
                                            // The page as the player would type it.
                                            &[("link", short_link(uri))],
                                        ),
                                    ),
                                )
                                .child(
                                    div()
                                        .font_family(theme::MONO)
                                        .font_weight(FontWeight::BOLD)
                                        .text_size(px(30.))
                                        .line_height(px(38.))
                                        .text_color(shell::c(n.ink))
                                        .child(user_code.to_string()),
                                ),
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
                )
            }
            Shown::Starting(what) => body.child(raised().child(
                shell::text(theme::night::UI, shell::c(n.ink_soft)).child(tr(what).to_string()),
            )),
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
                    let input_field = line_edit::mouse(input_field, cx, |this| &mut this.client_id_input);

                    body.child(
                        raised()
                            .flex()
                            .flex_col()
                            .gap(px(8.))
                            .child(
                                shell::text(theme::night::UI, shell::c(n.danger))
                                    .font_weight(FontWeight::BOLD)
                                    .child("Microsoft İstemci Kimliği (Client ID) Yapılandırması"),
                            )
                            .child(
                                shell::text(theme::night::CAPTION, shell::c(n.ink_soft)).child(
                                    "Microsoft girişi için Azure Uygulama Kimliği gereklidir:
                                     1. Azure Portal'a gidin: portal.azure.com > Microsoft Entra ID > Uygulama Kayıtları
                                     2. Yeni kayıt oluşturun: 'Herhangi bir kuruluş dizinindeki hesaplar ve kişisel Microsoft hesapları'
                                     3. Yeniden yönlendirme URI: Mobil ve masaüstü > https://login.microsoftonline.com/common/oauth2/nativeclient
                                     4. API İzinleri: XboxLive.signin ve offline_access izinlerini ekleyin
                                     5. Oluşturulan Uygulama (İstemci) Kimliği'ni aşağıdaki kutucuğa yapıştırın:",
                                ),
                            )
                            .child(input_field)
                            .child(
                                shell::text(theme::night::CAPTION, shell::c(n.muted)).child(
                                    "Girdiğiniz kimlik auth.json dosyasına kaydedilecek ve sonraki açılışlarda hatırlanacaktır.                                      Alternatif olarak aşağıdaki 'Demo / Çevrimdışı İncele' butonuyla yerel profille devam edebilirsiniz.",
                                ),
                            ),
                    )
                } else {
                    let msg = if e == "Sign-in cancelled." {
                        "Giriş kullanıcı tarafından iptal edildi.".to_string()
                    } else if e.contains("expired_token") || e.contains("timeout") || e.contains("timed out") {
                        "Giriş oturumu zaman aşımına uğradı. Lütfen tekrar deneyin.".to_string()
                    } else if e.contains("403") || e.contains("Forbidden") {
                        "Mojang API erişimi reddedildi (403). Azure uygulamasının Mojang onayı ve XboxLive.signin iznine sahip olduğundan emin olun.".to_string()
                    } else {
                        format!("Microsoft kimlik doğrulama hatası: {e}")
                    };
                    body.child(
                        raised()
                            .child(shell::text(theme::night::UI, shell::c(n.danger)).child(msg)),
                    )
                }
            }
            Shown::Idle => body,
        };
        let main = match state {
            Shown::Code(uri, _) => {
                let uri = uri.to_string();
                Some(
                    shell::action("owner-open", shell::Action::Main, &t)
                        .child(shell::label(tr("Open link").to_string()))
                        .on_click(move |_, _, cx| cx.open_url(&uri)),
                )
            }
            Shown::Starting(_) => None,
            Shown::Error(_) if !configured => Some(
                shell::action("owner-save-signin", shell::Action::Main, &t)
                    .child(shell::label("Kaydet ve Giriş Yap"))
                    .on_click(cx.listener(|this, _: &ClickEvent, _, cx| {
                        this.save_client_id_and_sign_in();
                        cx.notify();
                    })),
            ),
            _ => Some(
                microsoft::sign_in_button("owner-sign-in", &t).on_click(cx.listener(
                    |this, _: &ClickEvent, _, cx| {
                        this.sign_in_to_play();
                        cx.notify();
                    },
                )),
            ),
        };
        let waiting = matches!(state, Shown::Code(..)).then(|| {
            shell::text(theme::night::CAPTION, shell::c(n.muted))
                .relative()
                .child(tr("Waiting for Microsoft… the window goes on by itself.").to_string())
        });
        let mut buttons: Vec<AnyElement> = Vec::new();
        if let Some(m) = main {
            buttons.push(m.into_any_element());
        }
        buttons.push(
            shell::action("owner-demo", shell::Action::Raised, &t)
                .child(shell::label("Demo / Çevrimdışı İncele"))
                .on_click(cx.listener(|this, _: &ClickEvent, _, cx| {
                    this.start_demo_mode();
                    cx.notify();
                }))
                .into_any_element(),
        );
        buttons.push(
            shell::action("owner-later", shell::Action::Quiet, &t)
                .child(shell::label(tr("Not now").to_string()))
                .on_click(cx.listener(|this, _: &ClickEvent, _, cx| {
                    this.close_owner_screen();
                    cx.notify();
                }))
                .into_any_element(),
        );
        let answers = div()
            .relative()
            .flex()
            .flex_wrap()
            .items_center()
            .gap(px(10.))
            .mt(px(14.))
            .children(buttons);
        // Opaque: it sits over the page's content, not over the bare sky.
        let card = plate::cut(
            theme::night::PIXEL_CORNER,
            2,
            Some(plate::alpha(n.card, 0.98)),
            Some((plate::alpha(n.stroke, 2. * n.stroke_alpha), 1.)),
        )
        .flex()
        .flex_col()
        .gap(px(8.))
        // A set width: text measured at it wraps where it is drawn.
        .w(px(520.))
        .max_w_full()
        .p(px(32.))
        .child(body)
        .child(answers)
        .children(waiting);
        // The sky stays behind, dimmed: the sheet is over the launcher, not instead of it.
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
