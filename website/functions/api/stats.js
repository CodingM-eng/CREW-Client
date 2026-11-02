/**
 * Cloudflare Pages Function: /api/stats
 * Real CREW Client & CREW Network statistics endpoint.
 * Never outputs fake numbers or Horizon statistics.
 */
export async function onRequest(context) {
  const headers = {
    'Content-Type': 'application/json; charset=utf-8',
    'Access-Control-Allow-Origin': '*',
    'Cache-Control': 'public, max-age=60, s-maxage=60'
  };

  try {
    // 1. Fetch GitHub Release Downloads
    let downloads = {
      total: 0,
      setup: 0,
      portable: 0,
      latest_version: 'v2.1.0',
      available: false
    };

    try {
      const ghRes = await fetch('https://api.github.com/repos/CodingM-eng/CREW-Client/releases', {
        headers: {
          'User-Agent': 'CREW-Client-Stats/1.0',
          'Accept': 'application/vnd.github.v3+json'
        }
      });
      if (ghRes.ok) {
        const releases = await ghRes.json();
        if (Array.isArray(releases) && releases.length > 0) {
          downloads.latest_version = releases[0].tag_name || 'v2.1.0';
          let totalCount = 0;
          let setupCount = 0;
          let portableCount = 0;

          for (const rel of releases) {
            if (Array.isArray(rel.assets)) {
              for (const asset of rel.assets) {
                const count = asset.download_count || 0;
                totalCount += count;
                const name = (asset.name || '').toLowerCase();
                if (name.includes('setup') && name.endsWith('.exe')) {
                  setupCount += count;
                } else if (name.endsWith('.exe')) {
                  portableCount += count;
                }
              }
            }
          }
          downloads.total = totalCount;
          downloads.setup = setupCount;
          downloads.portable = portableCount;
          downloads.available = true;
        }
      }
    } catch (e) {
      // Graceful fallback: downloads.available remains false
    }

    // 2. Fetch Minecraft Server Status (oyna.crewnetwork.space)
    let server = {
      host: 'oyna.crewnetwork.space',
      port: 25565,
      online: false,
      players_online: 0,
      players_max: 0,
      version: '1.21.11 / Fabric',
      available: false
    };

    try {
      const mcRes = await fetch('https://api.mcstatus.io/v2/status/java/oyna.crewnetwork.space', {
        headers: {
          'User-Agent': 'CREW-Client-Stats/1.0'
        }
      });
      if (mcRes.ok) {
        const mcData = await mcRes.json();
        server.available = true;
        server.online = Boolean(mcData.online);
        if (mcData.online && mcData.players) {
          server.players_online = mcData.players.online || 0;
          server.players_max = mcData.players.max || 0;
        }
        if (mcData.version && mcData.version.name_clean) {
          server.version = mcData.version.name_clean;
        }
      }
    } catch (e) {
      // Graceful fallback
    }

    // 3. Assemble response adhering strictly to data honesty
    const payload = {
      product: 'CREW Client',
      brand: 'CREW Network',
      server_address: 'oyna.crewnetwork.space',
      timestamp: new Date().toISOString(),
      metrics: {
        downloads: {
          available: downloads.available,
          total: downloads.available ? downloads.total : null,
          setup_downloads: downloads.available ? downloads.setup : null,
          portable_downloads: downloads.available ? downloads.portable : null,
          latest_version: downloads.latest_version,
          status_text: downloads.available ? `${downloads.total}` : 'Veri şu anda kullanılamıyor'
        },
        server_players: {
          available: server.available,
          online: server.online,
          players_online: server.available && server.online ? server.players_online : (server.available ? 0 : null),
          players_max: server.available && server.online ? server.players_max : null,
          version: server.version,
          status_text: server.available ? (server.online ? `${server.players_online} / ${server.players_max}` : 'Sunucu Çevrimdışı (0 Oyuncu)') : 'Veri şu anda kullanılamıyor'
        },
        installations: {
          available: false,
          value: null,
          status_text: 'Veri şu anda kullanılamıyor',
          description: 'Kullanıcı gizliliğine tam saygı duyulmaktadır; yerel yükleme telemetrisi toplanmamaktadır.'
        },
        active_users: {
          available: false,
          value: null,
          status_text: 'Veri şu anda kullanılamıyor',
          description: 'İstemci zorunlu arka plan takibi yapmaz; gizlilik önceliklidir.'
        },
        registered_users: {
          available: false,
          value: null,
          status_text: 'Veri şu anda kullanılamıyor',
          description: 'Ayrı bir üyelik veritabanı tutulmaz; doğrudan resmi Microsoft hesabı kullanılır.'
        }
      }
    };

    return new Response(JSON.stringify(payload, null, 2), { status: 200, headers });
  } catch (err) {
    return new Response(JSON.stringify({
      error: 'İstatistikler alınamadı',
      message: err.message,
      timestamp: new Date().toISOString()
    }), { status: 500, headers });
  }
}
