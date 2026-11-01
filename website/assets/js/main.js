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

  // Dynamic GitHub Releases Loader (Graceful Fallback)
  const versionBadge = document.getElementById('latest-version-badge');
  const downloadSizeBadge = document.getElementById('release-size-badge');
  const sha256Badge = document.getElementById('release-sha-badge');

  const FALLBACK_DATA = {
    tag_name: "v2.1.0",
    size: "65.4 MB",
    sha256: "a9b3ab0cbbd31079aa14bcf7bf336490dbe8980edd19cc77acafdd6fe4e34212",
    download_url: "https://github.com/CodingM-eng/CREW-Client/releases"
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
    })
    .catch(() => {
      if (versionBadge) versionBadge.innerText = FALLBACK_DATA.tag_name;
      if (downloadSizeBadge) downloadSizeBadge.innerText = FALLBACK_DATA.size;
      if (sha256Badge) sha256Badge.innerText = FALLBACK_DATA.sha256;
    });
});
