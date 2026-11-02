/**
 * CREW Client — Canlı İstatistikler Betiği
 * Gerçek GitHub Release ve Minecraft sunucu API verilerini çeker.
 * Asla sahte oyuncu veya Horizon telemetrisi kullanmaz.
 */

document.addEventListener('DOMContentLoaded', () => {
  const GITHUB_REPO = 'CodingM-eng/CREW-Client';
  const SERVER_HOST = 'oyna.crewnetwork.space';

  const elDownloads = document.getElementById('stat-downloads');
  const elDownloadsBreakdown = document.getElementById('stat-downloads-breakdown');
  const elLatestTag = document.getElementById('stat-latest-tag');
  const elPlayers = document.getElementById('stat-players');
  const elPlayersDesc = document.getElementById('stat-players-desc');
  const elServerBadge = document.getElementById('stat-server-badge');
  const elServerDot = document.getElementById('server-status-dot');
  const elServerStatusBadge = document.getElementById('server-status-badge');
  const elServerStatusDesc = document.getElementById('server-status-desc');
  const elLastUpdated = document.getElementById('stat-last-updated');
  const btnRefresh = document.getElementById('btn-refresh-stats');

  let isFetching = false;

  async function fetchStats() {
    if (isFetching) return;
    isFetching = true;

    if (btnRefresh) {
      btnRefresh.disabled = true;
      btnRefresh.innerHTML = '<span>Yenileniyor...</span>';
    }

    try {
      // 1. Try internal /api/stats endpoint first
      let statsData = null;
      try {
        const res = await fetch('/api/stats', { cache: 'no-cache' });
        if (res.ok) {
          statsData = await res.json();
        }
      } catch (_) {
        // Fall back to direct external APIs
      }

      if (statsData && statsData.metrics) {
        applyStatsFromPayload(statsData);
      } else {
        // Fallback: Fetch directly from client
        await Promise.allSettled([
          fetchGitHubDownloads(),
          fetchMinecraftServerStatus()
        ]);
      }

      const now = new Date();
      if (elLastUpdated) {
        elLastUpdated.textContent = now.toLocaleTimeString('tr-TR', { hour: '2-digit', minute: '2-digit', second: '2-digit' });
      }
    } catch (err) {
      console.warn('İstatistik yükleme uyarısı:', err);
    } finally {
      isFetching = false;
      if (btnRefresh) {
        btnRefresh.disabled = false;
        btnRefresh.innerHTML = '<span>⟳ Verileri Yenile</span>';
      }
    }
  }

  function applyStatsFromPayload(data) {
    const m = data.metrics || {};

    // Downloads
    if (m.downloads && m.downloads.available) {
      if (elDownloads) elDownloads.textContent = `${m.downloads.total}`;
      if (elDownloadsBreakdown) {
        elDownloadsBreakdown.textContent = `Setup: ${m.downloads.setup_downloads || 0} indirme · Portable: ${m.downloads.portable_downloads || 0} indirme`;
      }
      if (elLatestTag) elLatestTag.textContent = m.downloads.latest_version || 'v2.1.0';
    } else {
      if (elDownloads) elDownloads.textContent = 'Veri şu anda kullanılamıyor';
      if (elDownloadsBreakdown) elDownloadsBreakdown.textContent = 'İndirme verisi geçici olarak alınamadı.';
    }

    // Minecraft Server
    if (m.server_players && m.server_players.available) {
      const isOnline = Boolean(m.server_players.online);
      updateServerUI(isOnline, m.server_players.players_online || 0, m.server_players.players_max || 0, m.server_players.version);
    } else {
      updateServerUI(false, 0, 0, null);
    }
  }

  async function fetchGitHubDownloads() {
    try {
      const res = await fetch(`https://api.github.com/repos/${GITHUB_REPO}/releases`, {
        headers: { 'Accept': 'application/vnd.github.v3+json' }
      });
      if (!res.ok) throw new Error(`HTTP ${res.status}`);
      const releases = await res.json();
      if (!Array.isArray(releases) || releases.length === 0) {
        throw new Error('Release bulunamadı');
      }

      let total = 0;
      let setup = 0;
      let portable = 0;
      const latestTag = releases[0].tag_name || 'v2.1.0';

      for (const rel of releases) {
        if (Array.isArray(rel.assets)) {
          for (const a of rel.assets) {
            const count = a.download_count || 0;
            total += count;
            const name = (a.name || '').toLowerCase();
            if (name.includes('setup') && name.endsWith('.exe')) {
              setup += count;
            } else if (name.endsWith('.exe')) {
              portable += count;
            }
          }
        }
      }

      if (elDownloads) elDownloads.textContent = `${total}`;
      if (elDownloadsBreakdown) {
        elDownloadsBreakdown.textContent = `Setup: ${setup} indirme · Portable: ${portable} indirme`;
      }
      if (elLatestTag) elLatestTag.textContent = latestTag;
    } catch (err) {
      if (elDownloads) elDownloads.textContent = 'Veri şu anda kullanılamıyor';
      if (elDownloadsBreakdown) {
        elDownloadsBreakdown.textContent = 'GitHub API kota sınırı nedeniyle indirme sayısı geçici olarak okunamadı.';
      }
    }
  }

  async function fetchMinecraftServerStatus() {
    try {
      const res = await fetch(`https://api.mcstatus.io/v2/status/java/${SERVER_HOST}`, {
        headers: { 'Accept': 'application/json' }
      });
      if (!res.ok) throw new Error(`HTTP ${res.status}`);
      const data = await res.json();

      const isOnline = Boolean(data.online);
      const onlinePlayers = (data.players && data.players.online) ? data.players.online : 0;
      const maxPlayers = (data.players && data.players.max) ? data.players.max : 0;
      const version = (data.version && data.version.name_clean) ? data.version.name_clean : '1.21.11';

      updateServerUI(isOnline, onlinePlayers, maxPlayers, version);
    } catch (err) {
      // Network failure or offline
      updateServerUI(false, 0, 0, null);
    }
  }

  function updateServerUI(online, players, max, version) {
    if (online) {
      if (elPlayers) elPlayers.textContent = `${players} / ${max}`;
      if (elPlayersDesc) elPlayersDesc.textContent = `${SERVER_HOST} sunucusunda şu anda aktif oynayan oyuncular.`;
      if (elServerBadge) {
        elServerBadge.className = 'stat-badge badge-live';
        elServerBadge.textContent = 'Çevrimiçi';
      }
      if (elServerDot) elServerDot.className = 'pulse-dot online';
      if (elServerStatusBadge) {
        elServerStatusBadge.className = 'stat-badge badge-live';
        elServerStatusBadge.textContent = 'Çevrimiçi';
      }
      if (elServerStatusDesc) {
        elServerStatusDesc.textContent = `Sunucu aktif · ${players} oyuncu bağlı · Sürüm: ${version || '1.21.11'}`;
      }
    } else {
      if (elPlayers) elPlayers.textContent = '0';
      if (elPlayersDesc) elPlayersDesc.textContent = 'Sunucu şu anda çevrimdışı veya bakımda.';
      if (elServerBadge) {
        elServerBadge.className = 'stat-badge badge-offline';
        elServerBadge.textContent = 'Çevrimdışı';
      }
      if (elServerDot) elServerDot.className = 'pulse-dot offline';
      if (elServerStatusBadge) {
        elServerStatusBadge.className = 'stat-badge badge-offline';
        elServerStatusBadge.textContent = 'Çevrimdışı';
      }
      if (elServerStatusDesc) {
        elServerStatusDesc.textContent = 'Sunucuya şu anda erişilemiyor. En kısa sürede tekrar çevrimiçi olacaktır.';
      }
    }
  }

  if (btnRefresh) {
    btnRefresh.addEventListener('click', fetchStats);
  }

  // Initial fetch
  fetchStats();
});
