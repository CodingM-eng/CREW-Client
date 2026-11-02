document.addEventListener('DOMContentLoaded', () => {
  // Mobile Nav Toggle
  const toggleBtn = document.querySelector('.mobile-toggle');
  const navLinks = document.querySelector('.nav-links');
  const navCta = document.querySelector('.nav-cta-group');

  if (toggleBtn) {
    toggleBtn.addEventListener('click', () => {
      const isVisible = navLinks.style.display === 'flex';
      navLinks.style.display = isVisible ? 'none' : 'flex';
      navLinks.style.flexDirection = 'column';
      navLinks.style.width = '100%';
      navLinks.style.marginTop = '1rem';
      if (navCta) {
        navCta.style.display = isVisible ? 'none' : 'flex';
        navCta.style.width = '100%';
        navCta.style.marginTop = '0.75rem';
      }
    });
  }

  // Copy buttons
  document.querySelectorAll('.btn-copy').forEach(btn => {
    btn.addEventListener('click', () => {
      const target = btn.getAttribute('data-copy');
      if (target) {
        navigator.clipboard.writeText(target).then(() => {
          const original = btn.innerText;
          btn.innerText = 'Kopyalandı!';
          btn.style.background = '#10b981';
          btn.style.borderColor = '#10b981';
          setTimeout(() => {
            btn.innerText = original;
            btn.style.background = '';
            btn.style.borderColor = '';
          }, 2000);
        });
      }
    });
  });

  // Documentation tab switcher
  const docButtons = document.querySelectorAll('.docs-nav-btn');
  const docArticles = document.querySelectorAll('.doc-article');

  docButtons.forEach(btn => {
    btn.addEventListener('click', () => {
      const targetId = btn.getAttribute('data-target');
      
      docButtons.forEach(b => b.classList.remove('active'));
      docArticles.forEach(a => a.classList.remove('active'));

      btn.classList.add('active');
      const article = document.getElementById(targetId);
      if (article) {
        article.classList.add('active');
      }
    });
  });

  // Dynamic GitHub Releases Loader & Direct Installer Resolution
  const versionBadge = document.getElementById('latest-version-badge');
  const downloadSizeBadge = document.getElementById('release-size-badge');
  const sha256Badge = document.getElementById('release-sha-badge');
  const installerButtons = document.querySelectorAll('.direct-installer-btn');

  const FALLBACK_DATA = {
    tag_name: "v2.1.0",
    installer_name: "CREW-Client-Setup-x64.exe",
    installer_size: "18.4 MB",
    installer_sha256: "68435d96281ae6f48cbe4574e13014b20afede8e768ce4b7d032deb561d2ed00",
    download_url: "https://github.com/CodingM-eng/CREW-Client/releases/download/v2.1.0/CREW-Client-Setup-x64.exe"
  };

  fetch('https://api.github.com/repos/CodingM-eng/CREW-Client/releases/latest')
    .then(res => {
      if (!res.ok) throw new Error('API request failed');
      return res.json();
    })
    .then(data => {
      if (versionBadge && data.tag_name) {
        versionBadge.innerText = data.tag_name;
      }

      // Find Windows x64 setup executable asset
      let installerAsset = null;
      if (data.assets && data.assets.length > 0) {
        installerAsset = data.assets.find(a => a.name.toLowerCase().includes('setup') && a.name.endsWith('.exe'))
          || data.assets.find(a => a.name.endsWith('.exe'));
      }

      if (installerAsset && installerAsset.browser_download_url) {
        installerButtons.forEach(btn => {
          btn.href = installerAsset.browser_download_url;
        });

        if (downloadSizeBadge) {
          const mbSize = (installerAsset.size / (1024 * 1024)).toFixed(1) + ' MB';
          downloadSizeBadge.innerText = mbSize;
        }
      } else {
        installerButtons.forEach(btn => {
          btn.href = FALLBACK_DATA.download_url;
        });
      }
    })
    .catch(() => {
      installerButtons.forEach(btn => {
        btn.href = FALLBACK_DATA.download_url;
      });
      if (versionBadge) versionBadge.innerText = FALLBACK_DATA.tag_name;
      if (downloadSizeBadge) downloadSizeBadge.innerText = FALLBACK_DATA.installer_size;
      if (sha256Badge) sha256Badge.innerText = FALLBACK_DATA.installer_sha256;
    });

  // Automatic YouTube Video Synchronization & Shorts Exclusion
  const youtubeContainer = document.getElementById('youtube-container');
  if (youtubeContainer) {
    const CHANNEL_ID = 'UCotr7s5r-QnbcO9FtX6reUw';
    const RSS_URL = `https://www.youtube.com/feeds/videos.xml?channel_id=${CHANNEL_ID}`;
    const API_URL = `https://api.rss2json.com/v1/api.json?rss_url=${encodeURIComponent(RSS_URL)}`;

    fetch(API_URL)
      .then(res => res.json())
      .then(data => {
        if (!data || data.status !== 'ok' || !Array.isArray(data.items) || data.items.length === 0) {
          return; // Keep default empty state
        }

        // Filter out YouTube Shorts
        const regularVideos = data.items.filter(item => {
          const title = (item.title || '').toLowerCase();
          const link = (item.link || '').toLowerCase();
          const desc = (item.description || '').toLowerCase();

          if (link.includes('/shorts/') || link.includes('shorts')) return false;
          if (title.includes('#shorts') || title.includes('#short') || title.includes('/shorts') || title.includes('(shorts)')) return false;
          if (desc.includes('#shorts') || desc.includes('#short')) return false;
          return true;
        });

        if (regularVideos.length === 0) {
          return; // Keep default empty state
        }

        // Render video grid
        let html = '<div class="youtube-grid">';
        regularVideos.slice(0, 6).forEach(video => {
          const vidId = video.guid ? video.guid.split(':').pop() : '';
          const watchUrl = video.link || `https://www.youtube.com/watch?v=${vidId}`;
          const thumb = video.thumbnail || (vidId ? `https://i.ytimg.com/vi/${vidId}/hqdefault.jpg` : '');
          const pubDate = video.pubDate ? video.pubDate.substring(0, 10) : '';

          html += `
            <div class="youtube-video-card">
              ${thumb ? `<img src="${thumb}" alt="${video.title}" class="youtube-video-thumb" loading="lazy">` : ''}
              <div class="youtube-video-info">
                <span class="youtube-video-date">${pubDate}</span>
                <h4 class="youtube-video-title">${video.title}</h4>
                <a href="${watchUrl}" target="_blank" rel="noopener" class="btn-primary" style="display: inline-flex; align-items: center; gap: 0.4rem; padding: 0.5rem 1rem; font-size: 0.85rem; margin-top: 0.5rem;">
                  <svg width="14" height="14" viewBox="0 0 24 24" fill="currentColor"><path d="M8 5v14l11-7z"/></svg>
                  <span>İzle</span>
                </a>
              </div>
            </div>
          `;
        });
        html += '</div>';

        youtubeContainer.innerHTML = html;
      })
      .catch(() => {
        // Silently preserve the elegant empty-state card
      });
  }

  // QR Code Modal Handlers
  const qrModal = document.getElementById('qr-modal');
  const btnShowQr = document.getElementById('btn-show-qr');
  const btnCloseQr = document.getElementById('btn-close-qr');
  const btnCopyQrLink = document.getElementById('btn-copy-qr-link');

  if (btnShowQr && qrModal) {
    btnShowQr.addEventListener('click', () => {
      qrModal.style.display = 'flex';
    });
  }

  if (btnCloseQr && qrModal) {
    btnCloseQr.addEventListener('click', () => {
      qrModal.style.display = 'none';
    });
  }

  if (qrModal) {
    qrModal.addEventListener('click', (e) => {
      if (e.target === qrModal) {
        qrModal.style.display = 'none';
      }
    });
  }

  if (btnCopyQrLink) {
    btnCopyQrLink.addEventListener('click', () => {
      navigator.clipboard.writeText('https://client.crewnetwork.space/').then(() => {
        const original = btnCopyQrLink.innerText;
        btnCopyQrLink.innerText = 'Kopyalandı!';
        btnCopyQrLink.style.background = '#10b981';
        btnCopyQrLink.style.borderColor = '#10b981';
        setTimeout(() => {
          btnCopyQrLink.innerText = original;
          btnCopyQrLink.style.background = '';
          btnCopyQrLink.style.borderColor = '';
        }, 2000);
      });
    });
  }
});
