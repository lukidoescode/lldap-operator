(function () {
  'use strict';

  // The script is served from <site-root>/<version>/src/version-selector.js.
  // Use its URL to derive the site root and current version, so paths work
  // regardless of whether docs are hosted at the domain root or a subpath.
  var scriptUrl = document.currentScript.src;
  var versionBase = new URL('..', scriptUrl).href;
  var siteRoot = new URL('..', versionBase).href;
  var currentVersion = versionBase.replace(/\/$/, '').split('/').pop() || '';
  var pageUrl = window.location.href;
  var currentPage = pageUrl.startsWith(versionBase) ? pageUrl.substring(versionBase.length) : '';

  fetch(new URL('versions.json', siteRoot).href)
    .then(function (res) {
      if (!res.ok) throw new Error(res.status);
      return res.json();
    })
    .then(function (data) {
      var select = document.createElement('select');
      select.id = 'version-selector';
      select.setAttribute('aria-label', 'Select documentation version');

      if (data.has_main) {
        var mainOpt = document.createElement('option');
        mainOpt.value = 'main-branch';
        mainOpt.textContent = 'main (dev)';
        if (currentVersion === 'main-branch') mainOpt.selected = true;
        select.appendChild(mainOpt);
      }

      data.versions.forEach(function (v) {
        var opt = document.createElement('option');
        opt.value = v;
        opt.textContent = v + (v === data.latest ? ' (latest)' : '');
        if (currentVersion === v) opt.selected = true;
        select.appendChild(opt);
      });

      if (select.options.length === 0) return;

      select.addEventListener('change', function () {
        window.location.href = new URL(this.value + '/' + currentPage, siteRoot).href;
      });

      var container = document.querySelector('.right-buttons');
      if (container) {
        container.insertBefore(select, container.firstChild);
      }
    })
    .catch(function () {
      // versions.json not available yet — no selector rendered
    });
})();
