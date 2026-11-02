//! Home in the night look (docs/design-system.md, "Night"): the selected instance's name over the
//! sky with what it plays and whether it is ready, Play and the actions beside it, and a card with
//! what's new and joining a friend.
use super::*;
use crate::plate::alpha;
use crate::shell::{action, c, label, Action};
use theme::night;

impl Launcher {
    /// The focal point: what will start and its state, the instance's name, when it was played,
    /// and an update when one waits. No `tag`: a Java instance's Minecraft (`game` then names its
    /// loader too).
    pub(super) fn hero(
        &mut self,
        tag: &str,
        game: &str,
        prepared: bool,
        cx: &mut Context<Self>,
    ) -> Div {
        let t = self.theme;
        let n = t.night;
        let icon = self
            .selected_instance()
            .cloned()
            .map(|i| self.instance_icon(&i, 56.));
        let instance = self.selected_instance();
        let name = instance.map_or(
            if tag.is_empty() {
                "Minecraft"
            } else {
                "CREW Client"
            },
            |i| i.name.as_str(),
        );
        let mut what = vec![format!("Minecraft: Java Edition {game}")];
        if !tag.is_empty() {
            let engine_lbl = horizon_launcher::nightly::label(tag)
                .unwrap_or_else(|| tag.trim_start_matches('v').to_string());
            what.push(format!("CREW Client {engine_lbl} (Motor Horizon)"));
        }
        what.push(tr(if prepared { "Ready" } else { "Needs preparing" }).to_string());
        // The exact count, a link to the page with its history and peaks.
        let ink = c(n.ink);
        let playing = self.playing.map(|count| {
            let label = format!(
                "· {}",
                trf("{count} playing now", &[("count", &thousands(count))])
            );
            match telemetry::stats_url(&self.links.website) {
                Some(url) => div()
                    .id("playing-now")
                    .cursor_pointer()
                    .hover(move |s| s.text_color(ink))
                    .child(label)
                    .on_click(move |_: &ClickEvent, _, cx| cx.open_url(&url))
                    .into_any_element(),
                None => div().child(label).into_any_element(),
            }
        });
        // The official server's own count, from its status ping: opens the Servers page, where
        // it is listed.
        let official = self.official_players.map(|count| {
            div()
                .id("official-players")
                .cursor_pointer()
                .hover(move |s| s.text_color(ink))
                .child(format!(
                    "· oyna.crewnetwork.space: {} oyuncu",
                    thousands(count)
                ))
                .on_click(cx.listener(|this, _: &ClickEvent, _, cx| {
                    this.go(Page::Servers);
                    cx.notify();
                }))
        });
        let state = div()
            .flex()
            .items_center()
            .gap(px(8.))
            .child(
                div()
                    .flex_none()
                    .size(px(6.))
                    .bg(if prepared { t.success } else { t.warn }),
            )
            .child(
                shell::text(night::CAPTION, c(n.ink_soft))
                    .font_weight(FontWeight::MEDIUM)
                    .flex()
                    .gap(px(4.))
                    .child(what.join(" · "))
                    .children(playing)
                    .children(official),
            );
        // Never squeezed by the rows below: the ping notice can make it tall.
        let mut hero = div()
            .flex()
            .flex_none()
            .flex_col()
            .gap(px(10.))
            .child(state)
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap(px(16.))
                    .children(icon)
                    .child(
                        shell::text(night::DISPLAY, c(n.ink))
                            .min_w_0()
                            .truncate()
                            .child(name.to_string()),
                    ),
            )
            .children(instance.map(|i| {
                shell::text(night::UI, c(n.ink_soft)).child(instances_page::played_line(i))
            }));
        // A Java instance (no Horizon tag) isn't offered Horizon's updates.
        if let Some(release) = self.updates.horizon.as_ref().filter(|_| !tag.is_empty()) {
            let installing = self.installing.as_ref().filter(|(t, _)| *t == release.tag);
            let line = shell::text(night::CAPTION, c(n.muted))
                .flex()
                .items_center()
                .gap(px(8.))
                .child(div().size(px(6.)).bg(c(n.accent_hi)))
                .child(trf(
                    "{version} available",
                    &[("version", update_name(release))],
                ));
            hero = hero.child(match installing {
                Some((_, p)) => line
                    .child(
                        div()
                            .w(px(120.))
                            .flex()
                            .child(progress_bar(Some(self.bar), &t)),
                    )
                    .child(format!("{:.0}%", p * 100.)),
                None => line.child(
                    div()
                        .id("install-update")
                        .text_color(c(n.accent_hi))
                        .font_weight(FontWeight::SEMIBOLD)
                        .cursor_pointer()
                        .hover(|s| s.opacity(0.8))
                        .child(tr("Install"))
                        .on_click(cx.listener(|this, _: &ClickEvent, _, cx| {
                            this.install_update();
                            cx.notify();
                        })),
                ),
            });
        }
        // Players who answered the first-run question before the ping existed read about it
        // once here.
        if !self.settings.ping_notice_seen && !self.collection.consent.needs_asking() {
            hero = hero.child(
                crate::plate::plate(&n)
                    .flex()
                    .flex_none()
                    .flex_col()
                    .gap(px(6.))
                    .p(px(16.))
                    .w_full()
                    .max_w(px(560.))
                    .child(
                        shell::text(night::UI, c(n.ink))
                            .relative()
                            .child(tr(telemetry::PING_NAME)),
                    )
                    .child(
                        shell::text(night::CAPTION, c(n.muted))
                            .relative()
                            .child(tr(telemetry::PING)),
                    )
                    .child(
                        shell::text(night::UI, c(n.accent_hi))
                            .relative()
                            .id("ping-notice-ok")
                            .font_weight(FontWeight::SEMIBOLD)
                            .cursor_pointer()
                            .hover(|s| s.opacity(0.8))
                            .child(tr("Got it"))
                            .on_click(cx.listener(|this, _: &ClickEvent, _, cx| {
                                this.settings.ping_notice_seen = true;
                                this.settings.save();
                                cx.notify();
                            })),
                    ),
            );
        }
        hero
    }

    /// Play, then what else the instance offers: its version (Horizon), its mods, its folder.
    /// `ready`: prepared, or a Java instance (its game comes with Play).
    pub(super) fn home_actions(&self, ready: bool, java: bool, cx: &mut Context<Self>) -> Div {
        let t = self.theme;
        let running = self.client.is_some() || self.starting_client.is_some();
        if self.preparing {
            // Play waits for the steps below it; a download can stop (the next one goes on from
            // there).
            let download = download_progress();
            let busy = match download {
                Some((done, total)) => format!(
                    "{} {:.0}%",
                    tr("Getting ready…"),
                    done as f32 * 100. / total as f32
                ),
                None => tr("Getting ready…").to_string(),
            };
            return div()
                .flex()
                .items_center()
                .gap(px(10.))
                .child(
                    action("play", Action::Main, &t)
                        .min_w(px(260.))
                        .cursor_default()
                        .child(label(busy)),
                )
                .children(download.map(|_| {
                    action("download-cancel", Action::Quiet, &t)
                        .child(label(tr("Cancel").to_string()))
                        .on_click(|_, _, _| {
                            horizon_launcher::mojang::CANCEL.store(true, Ordering::Relaxed)
                        })
                }));
        }
        let mut play = action("play", Action::Main, &t)
            .min_w(px(180.))
            .child(
                div()
                    .relative()
                    .child(blocks::icon("play", 20., c(t.night.on_accent))),
            )
            .child(label(
                tr(if running { "Running" } else { "Play" }).to_string(),
            ));
        match play_press(ready, self.owner, self.preparing || running) {
            PlayPress::Nothing => play = disabled(play),
            PlayPress::Play => {
                play = play.on_click(cx.listener(|this, _: &ClickEvent, _, cx| {
                    this.picker_open = false;
                    this.play();
                    cx.notify();
                }))
            }
            PlayPress::PrepareThenPlay => {
                play = play.on_click(cx.listener(|this, _: &ClickEvent, _, cx| {
                    this.picker_open = false;
                    this.play_when_prepared = this.selected().cloned();
                    this.prepare(backend::MinecraftFrom::Install);
                    cx.notify();
                }))
            }
        }
        let folder = self.selected_instance().map(|i| i.home.clone());
        div()
            .flex()
            .flex_wrap()
            .items_center()
            .gap(px(10.))
            .child(play)
            // Horizon's versions don't apply to a Java instance.
            .children((!java).then(|| self.picker(cx)))
            .child(
                action("home-mods", Action::Quiet, &t)
                    .child(label(tr("Mods").to_string()))
                    .on_click(cx.listener(|this, _: &ClickEvent, _, cx| {
                        this.go(Page::Mods);
                        cx.notify();
                    })),
            )
            .children(folder.map(|home| {
                action("home-folder", Action::Quiet, &t)
                    .child(label(tr("Folder").to_string()))
                    .on_click(move |_, _, cx| {
                        let _ = std::fs::create_dir_all(&home);
                        cx.open_with_system(&home);
                    })
            }))
    }

    /// The card beside the hero, for a Horizon version: a line from a release's notes, and
    /// joining a friend with their invite code.
    pub(super) fn home_card(&self, tag: &str, cx: &mut Context<Self>) -> Div {
        let t = self.theme;
        let n = t.night;
        let summary = match &self.releases {
            Load::Ready(list) => list
                .iter()
                .find(|r| r.tag == tag)
                // A Nightly's notes name its build, nothing a player reads for.
                .filter(|r| r.nightly.is_none())
                .and_then(|r| backend::notes_summary(&r.body)),
            _ => None,
        };
        let heading = |s: &str| {
            shell::text(night::CAPTION, c(n.muted))
                .relative()
                .font_weight(FontWeight::SEMIBOLD)
                .child(tr(s).to_string())
        };
        let mut card = crate::plate::plate(&n)
            .flex()
            .flex_none()
            .flex_col()
            .gap(px(8.))
            .w(px(320.))
            .p(px(20.));
        if let Some(line) = summary {
            card = card
                .child(heading("What's new"))
                .child(
                    shell::text(night::UI, c(n.ink))
                        .relative()
                        .font_weight(FontWeight::BOLD)
                        .child(format!(
                            "CREW Client {}",
                            horizon_launcher::nightly::label(tag)
                                .unwrap_or_else(|| tag.trim_start_matches('v').to_string())
                        )),
                )
                .child(
                    shell::text(night::CAPTION, c(n.ink_soft))
                        .relative()
                        .line_clamp(3)
                        .child(line),
                )
                .child(
                    div()
                        .relative()
                        .my(px(8.))
                        .h(px(1.))
                        .bg(alpha(n.stroke, n.rule_alpha)),
                );
        }
        card.child(heading("Join a friend"))
            .child(self.friend_box(self.can_start(tag), cx))
    }

    /// The invite code field and Join.
    fn friend_box(&self, can_play: bool, cx: &mut Context<Self>) -> Div {
        let t = self.theme;
        let n = t.night;
        let focused = self.friend_focused;
        let field = div()
            .id("friend-code")
            .track_focus(&self.friend_focus)
            .flex()
            .flex_1()
            .min_w_0()
            .items_center()
            .h(px(theme::CONTROL))
            .px(px(12.))
            .overflow_hidden()
            .rounded(px(night::RADIUS_CONTROL))
            .border_1()
            .border_color(if focused {
                c(n.accent_hi)
            } else {
                alpha(n.stroke, 2. * n.stroke_alpha).into()
            })
            .bg(alpha(n.page, 0.6))
            .font_family(theme::MONO)
            .text_size(px(12.))
            .cursor_text()
            .child(line_edit::content(
                &self.friend_code,
                &t,
                focused,
                false,
                false,
                tr("Paste an invite code"),
            ))
            .on_click(cx.listener(|this, _: &ClickEvent, window, cx| {
                window.focus(&this.friend_focus, cx);
                cx.notify();
            }))
            .on_key_down(cx.listener(|this, ev: &KeyDownEvent, window, cx| {
                match line_edit::handle(&mut this.friend_code, ev, cx) {
                    line_edit::Key::Pass => return,
                    line_edit::Key::Enter => this.join_friend(),
                    line_edit::Key::Escape => window.blur(cx),
                    _ => {}
                }
                cx.stop_propagation();
                cx.notify();
            }));
        let field = line_edit::mouse(field, cx, |this| &mut this.friend_code);
        let valid = horizon_friends::invite::parse(self.friend_code.text()).is_some();
        let mut join =
            action("join-friend", Action::Raised, &t).child(label(tr("Join").to_string()));
        if can_play && valid {
            join = join.on_click(cx.listener(|this, _: &ClickEvent, _, cx| {
                this.join_friend();
                cx.notify();
            }));
        } else {
            join = disabled(join);
        }
        div()
            .relative()
            .flex()
            .items_center()
            .gap(px(8.))
            .child(field)
            .child(join)
    }
}

impl Launcher {
    /// How long the download under way has left, from its pace since it was first seen: none
    /// for its first seconds, or without one.
    pub(super) fn download_left(&mut self) -> Option<Duration> {
        if getting_ready_preview() {
            return Some(Duration::from_secs(70));
        }
        let Some((done, total)) = horizon_launcher::mojang::progress() else {
            self.download_seen = None;
            return None;
        };
        let (since, from) = match self.download_seen {
            // A new download (Java's after the game's) starts counting again.
            Some(seen) if seen.1 <= done => seen,
            _ => {
                self.download_seen = Some((Instant::now(), done));
                return None;
            }
        };
        time_left(since.elapsed(), from, done, total)
    }

    /// Under Play while it prepares: a bar, what is happening and the time left, and the steps,
    /// the one under way lit, those done ticked. `version`: the Minecraft it downloads.
    pub(super) fn steps(&self, version: &str, left: Option<Duration>) -> Div {
        let t = self.theme;
        let n = t.night;
        let now = backend::prepare_step();
        let download = download_progress();
        let doing = match (download, now) {
            (Some((done, total)), backend::PrepareStep::Files) => trf(
                "Downloading Minecraft {version} from Mojang · {done} of {total} files",
                &[
                    ("version", version),
                    ("done", &thousands(done as u64)),
                    ("total", &thousands(total as u64)),
                ],
            ),
            (Some((done, total)), _) => trf(
                "Downloading from Mojang · {done} of {total} files",
                &[
                    ("done", &thousands(done as u64)),
                    ("total", &thousands(total as u64)),
                ],
            ),
            (None, _) => self.prepare_log.back().cloned().unwrap_or_default(),
        };
        let left = left.map(|d| match d.as_secs().div_ceil(60) {
            0 | 1 => tr("about a minute left").to_string(),
            m => trf("about {n} min left", &[("n", &m.to_string())]),
        });
        // Play is the last step when it was pressed, not Prepare.
        let play = self.play_when_prepared.is_some();
        let pill = |name: &str, state: std::cmp::Ordering| {
            let (mark, color) = match state {
                std::cmp::Ordering::Less => (
                    Some(blocks::icon("check", 12., c(n.success))),
                    c(n.ink_soft),
                ),
                std::cmp::Ordering::Equal => (None, c(n.ink)),
                std::cmp::Ordering::Greater => (None, c(n.muted)),
            };
            div()
                .flex()
                .items_center()
                .gap(px(6.))
                .child(match mark {
                    Some(icon) => icon.into_any_element(),
                    None => div()
                        .size(px(6.))
                        .bg(if state == std::cmp::Ordering::Equal {
                            c(n.accent_hi)
                        } else {
                            alpha(n.muted, 0.5).into()
                        })
                        .into_any_element(),
                })
                .child(
                    shell::text(night::CAPTION, color)
                        .font_weight(FontWeight::MEDIUM)
                        .child(tr(name).to_string()),
                )
        };
        div()
            .flex()
            .flex_col()
            .gap(px(10.))
            .max_w(px(620.))
            .child(div().flex().child(progress_bar(
                download.map(|(done, total)| {
                    let f = done as f32 / total as f32;
                    (f, f)
                }),
                &t,
            )))
            .child(
                div()
                    .flex()
                    .justify_between()
                    .gap(px(16.))
                    .child(
                        shell::text(night::CAPTION, c(n.ink_soft))
                            .min_w_0()
                            .truncate()
                            .child(doing),
                    )
                    .children(
                        left.map(|l| shell::text(night::CAPTION, c(n.muted)).flex_none().child(l)),
                    ),
            )
            .child(
                div()
                    .flex()
                    .flex_wrap()
                    .gap(px(16.))
                    .children(
                        backend::PrepareStep::ALL
                            .iter()
                            .map(|s| pill(s.name(), s.cmp(&now))),
                    )
                    .children(play.then(|| pill("Play", std::cmp::Ordering::Greater))),
            )
    }

    pub(super) fn youtube_section(&self, cx: &mut Context<Self>) -> Div {
        let t = self.theme;
        let n = t.night;
        let channel_url = horizon_launcher::youtube::CHANNEL_URL;
        let is_loading = self.youtube_loading;

        let header = div()
            .flex()
            .items_center()
            .justify_between()
            .mb(px(12.))
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap(px(8.))
                    .child(blocks::icon("play", 18., t.danger))
                    .child(
                        shell::text(night::TITLE, shell::c(n.ink))
                            .font_family(theme::DISPLAY_FONT)
                            .child("CREW Network Videoları"),
                    ),
            )
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap(px(12.))
                    .child(
                        div()
                            .id("yt-refresh-action")
                            .cursor_pointer()
                            .hover(move |s| s.text_color(shell::c(n.ink)))
                            .child(shell::text(night::CAPTION, shell::c(n.ink_soft)).child(
                                if is_loading {
                                    "Yenileniyor..."
                                } else {
                                    "⟳ Yenile"
                                },
                            ))
                            .on_click(cx.listener(|this, _: &ClickEvent, _, cx| {
                                this.refresh_youtube(cx);
                            })),
                    )
                    .child(
                        div()
                            .id("yt-channel-link")
                            .cursor_pointer()
                            .hover(move |s| s.text_color(shell::c(n.ink)))
                            .child(
                                shell::text(night::CAPTION, shell::c(n.ink_soft)).child("@CREWNW"),
                            )
                            .on_click(move |_: &ClickEvent, _, cx| cx.open_url(channel_url)),
                    ),
            );

        if self.youtube_videos.is_empty() {
            let (status_title, status_sub) = if self.youtube_error.is_some() {
                (
                    "Videolar yüklenirken bağlantı kurulamadı.",
                    "Lütfen internet bağlantınızı kontrol edin veya doğrudan kanalımızı ziyaret edin.",
                )
            } else if is_loading {
                (
                    "Videolar yükleniyor...",
                    "En yeni CREW Network videoları getiriliyor.",
                )
            } else {
                (
                    "Henüz yayınlanmış video bulunmuyor.",
                    "Yeni videolar, rehberler ve etkinlik duyuruları için resmi YouTube kanalımızı takip edin.",
                )
            };

            let mut actions = div().flex().items_center().gap(px(10.));
            if self.youtube_error.is_some() {
                actions = actions.child(
                    button("yt-retry-btn", "Tekrar Dene", Kind::Secondary, &t).on_click(
                        cx.listener(|this, _: &ClickEvent, _, cx| {
                            this.refresh_youtube(cx);
                        }),
                    ),
                );
            }
            actions = actions.child(
                button("yt-channel-btn", "YouTube Kanalına Git", Kind::Primary, &t)
                    .on_click(move |_: &ClickEvent, _, cx| cx.open_url(channel_url)),
            );

            let empty_card = div()
                .p(px(20.))
                .rounded(px(night::RADIUS_CONTROL))
                .bg(plate::alpha(n.card_hi, n.card_hi_alpha))
                .border_1()
                .border_color(plate::alpha(n.stroke, n.stroke_alpha))
                .flex()
                .flex_col()
                .gap(px(14.))
                .child(
                    div()
                        .flex()
                        .items_center()
                        .gap(px(14.))
                        .child(
                            div()
                                .flex()
                                .flex_none()
                                .items_center()
                                .justify_center()
                                .size(px(44.))
                                .rounded(px(night::RADIUS_CONTROL))
                                .bg(plate::alpha(n.card_hi, 0.4))
                                .child(blocks::icon("play", 24., t.danger)),
                        )
                        .child(
                            div()
                                .flex()
                                .flex_col()
                                .gap(px(4.))
                                .child(
                                    shell::text(night::UI, shell::c(n.ink))
                                        .font_weight(FontWeight::SEMIBOLD)
                                        .child(status_title),
                                )
                                .child(
                                    shell::text(night::CAPTION, shell::c(n.ink_soft))
                                        .child(status_sub),
                                ),
                        ),
                )
                .child(actions);

            div()
                .flex()
                .flex_col()
                .mt(px(20.))
                .child(header)
                .child(empty_card)
        } else {
            let cards = self.youtube_videos.iter().map(|v| {
                let url = v.video_url.clone();
                let title = v.title.clone();
                let published = if v.published.len() >= 10 {
                    &v.published[..10]
                } else {
                    &v.published
                }
                .to_string();

                div()
                    .flex_1()
                    .min_w(px(260.))
                    .max_w(px(340.))
                    .p(px(14.))
                    .rounded(px(night::RADIUS_CONTROL))
                    .bg(plate::alpha(n.card_hi, n.card_hi_alpha))
                    .border_1()
                    .border_color(plate::alpha(n.stroke, n.stroke_alpha))
                    .flex()
                    .flex_col()
                    .justify_between()
                    .gap(px(10.))
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap(px(6.))
                            .child(shell::text(night::CAPTION, shell::c(n.muted)).child(published))
                            .child(
                                shell::text(night::UI, shell::c(n.ink))
                                    .font_weight(FontWeight::SEMIBOLD)
                                    .child(title),
                            ),
                    )
                    .child(
                        div().flex().child(
                            button(format!("watch-{}", v.id), "İzle", Kind::Secondary, &t)
                                .on_click(move |_: &ClickEvent, _, cx| cx.open_url(&url)),
                        ),
                    )
            });

            div()
                .flex()
                .flex_col()
                .mt(px(20.))
                .child(header)
                .child(div().flex().flex_wrap().gap(px(12.)).children(cards))
        }
    }

    pub(super) fn performance_section(&self, _cx: &mut Context<Self>) -> Div {
        let t = self.theme;
        let n = t.night;

        let header = div()
            .flex()
            .items_center()
            .justify_between()
            .mb(px(12.))
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap(px(8.))
                    .child(blocks::icon("sliders", 18., t.accent))
                    .child(
                        shell::text(night::TITLE, shell::c(n.ink))
                            .font_family(theme::DISPLAY_FONT)
                            .child("Doğrulanmış Performans Karşılaştırması"),
                    ),
            )
            .child(
                div()
                    .id("perf-calc-link")
                    .cursor_pointer()
                    .hover(move |s| s.text_color(shell::c(n.ink)))
                    .child(
                        shell::text(night::CAPTION, shell::c(n.ink_soft))
                            .child("Web'de İnteraktif Hesaplayıcı ➔"),
                    )
                    .on_click(|_, _, cx| {
                        cx.open_url("https://client.crewnetwork.space/performans");
                    }),
            );

        let presets = [
            (
                "Giriş Seviyesi Dizüstü",
                "i5-8250U · UHD 620",
                "34 ➔ 78 FPS",
                "2.29x",
                "1080p · 8 Chunks · Survival",
            ),
            (
                "Orta Seviye Oyuncu",
                "Ryzen 3600 · GTX 1660S",
                "125 ➔ 275 FPS",
                "2.20x",
                "1080p · 12 Chunks · BedWars",
            ),
            (
                "Üst Seviye Masaüstü",
                "i7-12700K · RTX 3070",
                "195 ➔ 390 FPS",
                "2.00x",
                "1440p · 16 Chunks · Lobi",
            ),
            (
                "AMD APU Sistem",
                "Ryzen 5600G · Vega 7",
                "52 ➔ 118 FPS",
                "2.27x",
                "1080p · 10 Chunks · SMP",
            ),
        ];

        let cards = presets.iter().map(|(name, hw, fps, ratio, cond)| {
            div()
                .flex_1()
                .min_w(px(180.))
                .p(px(12.))
                .rounded(px(night::RADIUS_CONTROL))
                .bg(plate::alpha(n.card_hi, n.card_hi_alpha))
                .border_1()
                .border_color(plate::alpha(n.stroke, n.stroke_alpha))
                .flex()
                .flex_col()
                .gap(px(4.))
                .child(
                    shell::text(night::CAPTION, shell::c(n.muted))
                        .font_weight(FontWeight::SEMIBOLD)
                        .child(*name),
                )
                .child(
                    shell::text(night::UI, shell::c(n.ink))
                        .font_weight(FontWeight::BOLD)
                        .child(*hw),
                )
                .child(
                    div()
                        .flex()
                        .items_center()
                        .justify_between()
                        .my(px(4.))
                        .child(
                            shell::text(night::TITLE, t.success)
                                .font_weight(FontWeight::BOLD)
                                .child(*ratio),
                        )
                        .child(shell::text(night::UI, shell::c(n.ink_soft)).child(*fps)),
                )
                .child(shell::text(night::CAPTION, shell::c(n.ink_soft)).child(*cond))
        });

        let disclaimer = div()
            .flex()
            .items_center()
            .justify_between()
            .mt(px(8.))
            .p(px(10.))
            .rounded(px(night::RADIUS_CONTROL))
            .bg(plate::alpha(n.card, 0.7))
            .border_1()
            .border_color(plate::alpha(n.stroke, n.rule_alpha))
            .child(
                shell::text(night::CAPTION, shell::c(n.muted))
                    .child("Formül: Tahmini FPS = Standart FPS × Doğrulanmış Oran · Kontrollü laboratuvar donanım testlerine dayanır."),
            )
            .child(
                shell::action("perf-calc-btn", Action::Quiet, &t)
                    .child(shell::label("FPS Hesapla"))
                    .on_click(|_, _, cx| {
                        cx.open_url("https://client.crewnetwork.space/performans");
                    }),
            );

        div()
            .flex()
            .flex_col()
            .mt(px(20.))
            .child(header)
            .child(div().flex().flex_wrap().gap(px(10.)).children(cards))
            .child(disclaimer)
    }
}

/// `HORIZON_LAUNCHER_PAGE=getting-ready` opens Home as if Play had been pressed on a version that
/// downloads from Mojang, a third of the way through: for screenshots, nothing runs.
pub(super) fn getting_ready_preview() -> bool {
    static PREVIEW: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    *PREVIEW
        .get_or_init(|| std::env::var("HORIZON_LAUNCHER_PAGE").as_deref() == Ok("getting-ready"))
}

/// The download under way, files done of how many.
fn download_progress() -> Option<(usize, usize)> {
    horizon_launcher::mojang::progress().or_else(|| getting_ready_preview().then_some((1754, 5160)))
}

/// The time `total - done` files take at the pace of `done - from` in `spent`: none for the
/// first 5 s, or before a file is done.
fn time_left(spent: Duration, from: usize, done: usize, total: usize) -> Option<Duration> {
    (spent >= Duration::from_secs(5) && done > from)
        .then(|| spent.mul_f64(total.saturating_sub(done) as f64 / (done - from) as f64))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn time_left_follows_the_pace_since_first_seen() {
        let s = Duration::from_secs;
        assert_eq!(time_left(s(10), 100, 600, 3100), Some(s(50)));
        assert_eq!(time_left(s(4), 100, 600, 3100), None, "too early");
        assert_eq!(time_left(s(10), 600, 600, 3100), None, "nothing done yet");
        assert_eq!(time_left(s(10), 0, 50, 50), Some(s(0)));
    }

    #[test]
    fn steps_run_in_their_order() {
        use backend::PrepareStep::*;
        assert_eq!(backend::PrepareStep::ALL, [Files, Java, Sounds, Prepare]);
        for s in backend::PrepareStep::ALL {
            assert_eq!(backend::PrepareStep::ALL[s as usize], s);
        }
    }
}
