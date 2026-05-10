(function () {
  'use strict';

  var pathSegments = window.location.pathname.split('/');
  var currentVersion = pathSegments[1] || '';
  var currentPage = pathSegments.slice(2).join('/');

  fetch('/versions.json')
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
        window.location.href = '/' + this.value + '/' + currentPage;
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
