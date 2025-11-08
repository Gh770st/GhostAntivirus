// GhostAntivirus Browser Extension - Content Script
class GhostAntivirusContent {
  constructor() {
    this.initialize();
  }

  initialize() {
    console.log('GhostAntivirus Content Script loaded for:', window.location.href);
    
    // Inject our security monitoring script
    this.injectSecurityScript();
    
    // Set up message listener
    this.setupMessageListener();
    
    // Start monitoring
    this.startMonitoring();
  }

  injectSecurityScript() {
    const script = document.createElement('script');
    script.src = chrome.runtime.getURL('src/injected.js');
    script.onload = () => script.remove();
    (document.head || document.documentElement).appendChild(script);
  }

  setupMessageListener() {
    chrome.runtime.onMessage.addListener((request, sender, sendResponse) => {
      switch (request.action) {
        case 'pageScanResult':
          this.displayScanResult(request.data);
          break;
        case 'highlightThreats':
          this.highlightThreatElements();
          break;
        case 'removeHighlights':
          this.removeHighlights();
          break;
      }
      return true;
    });
  }

  startMonitoring() {
    // Monitor for suspicious content
    this.monitorForms();
    this.monitorLinks();
    this.monitorScripts();
    this.monitorDownloads();
  }

  monitorForms() {
    const forms = document.querySelectorAll('form');
    
    forms.forEach(form => {
      // Check for suspicious form attributes
      if (this.isSuspiciousForm(form)) {
        this.flagSuspiciousElement(form, 'Suspicious form detected');
      }

      // Monitor form submission
      form.addEventListener('submit', (e) => {
        if (this.isPhishingAttempt(form)) {
          e.preventDefault();
          this.warnPhishingAttempt();
        }
      });
    });
  }

  monitorLinks() {
    const links = document.querySelectorAll('a[href]');
    
    links.forEach(link => {
      const href = link.getAttribute('href');
      
      // Check for suspicious links
      if (this.isSuspiciousLink(href)) {
        this.flagSuspiciousLink(link);
      }

      // Add warning for external links
      if (href && (href.startsWith('http://') || href.startsWith('https://'))) {
        if (!this.isSameOrigin(href)) {
          link.addEventListener('click', (e) => {
            this.warnExternalLink(href, e);
          });
        }
      }
    });
  }

  monitorScripts() {
    const scripts = document.querySelectorAll('script[src]');
    
    scripts.forEach(script => {
      const src = script.getAttribute('src');
      
      if (this.isSuspiciousScript(src)) {
        this.flagSuspiciousElement(script, 'Suspicious script detected');
        console.warn('Suspicious script detected:', src);
      }
    });
  }

  monitorDownloads() {
    const downloadLinks = document.querySelectorAll('a[download]');
    
    downloadLinks.forEach(link => {
      link.addEventListener('click', (e) => {
        const filename = link.getAttribute('download') || 'unknown';
        this.warnDownload(filename, e);
      });
    });
  }

  isSuspiciousForm(form) {
    // Check for suspicious attributes
    const action = form.getAttribute('action') || '';
    const method = form.getAttribute('method') || '';
    
    // Check for suspicious patterns
    const suspiciousPatterns = [
      /login/i,
      /signin/i,
      /password/i,
      /credential/i,
      /account/i
    ];

    const hasSensitiveFields = Array.from(form.elements).some(element => {
      const name = element.name || '';
      const type = element.type || '';
      return suspiciousPatterns.some(pattern => 
        pattern.test(name) || pattern.test(type)
      );
    });

    const isExternalAction = action && !this.isSameOrigin(action);
    
    return hasSensitiveFields && isExternalAction;
  }

  isPhishingAttempt(form) {
    const action = form.getAttribute('action') || '';
    const inputs = Array.from(form.elements);
    
    // Check for password fields pointing to external domains
    const hasPasswordField = inputs.some(input => input.type === 'password');
    
    if (hasPasswordField && action && !this.isSameOrigin(action)) {
      return true;
    }
    
    // Check for common phishing domain patterns
    const domain = this.extractDomain(action);
    const suspiciousDomains = [
      'update-account',
      'verify-account',
      'secure-login',
      'auth-confirm'
    ];
    
    return suspiciousDomains.some(pattern => domain.includes(pattern));
  }

  isSuspiciousLink(href) {
    if (!href) return false;
    
    // Check for suspicious URL patterns
    const suspiciousPatterns = [
      /bit\.ly/,
      /tinyurl\.com/,
      /t\.co/,
      /shortened/,
      /redirect/,
      /download\.exe$/,
      /\.exe$/i,
      /\.scr$/i,
      /\.bat$/i,
      /\.com$/i
    ];

    return suspiciousPatterns.some(pattern => pattern.test(href));
  }

  isSuspiciousScript(src) {
    if (!src) return false;
    
    // Check for suspicious script sources
    const suspiciousPatterns = [
      /eval\(/,
      /document\.write/,
      /innerHTML/,
      /outerHTML/,
      /\.js\?random=/,
      /analytics/,
      /tracking/,
      /fingerprint/
    ];

    return suspiciousPatterns.some(pattern => pattern.test(src));
  }

  isSameOrigin(url) {
    try {
      return new URL(url, window.location.origin).origin === window.location.origin;
    } catch (error) {
      return false;
    }
  }

  extractDomain(url) {
    try {
      return new URL(url, window.location.origin).hostname;
    } catch (error) {
      return '';
    }
  }

  flagSuspiciousElement(element, message) {
    element.setAttribute('data-ghost-antivirus-warning', message);
    element.style.border = '2px solid #ff9800';
    element.style.boxShadow = '0 0 10px rgba(255, 152, 0, 0.5)';
  }

  flagSuspiciousLink(link) {
    link.style.color = '#ff9800';
    link.style.textDecoration = 'underline wavy #ff9800';
    
    // Add warning tooltip
    link.title = '⚠️ Suspicious link detected by GhostAntivirus';
  }

  highlightThreatElements() {
    const flaggedElements = document.querySelectorAll('[data-ghost-antivirus-warning]');
    
    flaggedElements.forEach(element => {
      element.style.transition = 'all 0.3s ease';
      element.style.animation = 'ghostPulse 2s infinite';
    });

    // Add pulse animation
    const style = document.createElement('style');
    style.textContent = `
      @keyframes ghostPulse {
        0%, 100% { opacity: 1; }
        50% { opacity: 0.7; }
      }
    `;
    document.head.appendChild(style);
  }

  removeHighlights() {
    const flaggedElements = document.querySelectorAll('[data-ghost-antivirus-warning]');
    
    flaggedElements.forEach(element => {
      element.style.border = '';
      element.style.boxShadow = '';
      element.style.color = '';
      element.style.textDecoration = '';
      element.style.animation = '';
      element.removeAttribute('data-ghost-antivirus-warning');
    });
  }

  warnPhishingAttempt() {
    const warning = this.createWarningOverlay(
      '🛑 Phishing Attempt Detected',
      'This form appears to be a phishing attempt. GhostAntivirus has blocked this submission to protect your credentials.',
      'danger'
    );
    
    document.body.appendChild(warning);
  }

  warnExternalLink(url, event) {
    event.preventDefault();
    
    const warning = this.createWarningOverlay(
      '⚠️ External Link',
      `You are about to navigate to an external website: ${this.extractDomain(url)}`,
      'warning',
      () => {
        window.open(url, '_blank');
        this.removeWarningOverlay();
      }
    );
    
    document.body.appendChild(warning);
  }

  warnDownload(filename, event) {
    event.preventDefault();
    
    const warning = this.createWarningOverlay(
      '⬇️ Download Warning',
      `You are about to download: ${filename}. Please ensure this file is from a trusted source.`,
      'warning',
      () => {
        event.target.click();
        this.removeWarningOverlay();
      }
    );
    
    document.body.appendChild(warning);
  }

  createWarningOverlay(title, message, type = 'warning', onContinue = null) {
    const overlay = document.createElement('div');
    overlay.id = 'ghost-antivirus-warning-overlay';
    overlay.style.cssText = `
      position: fixed;
      top: 0;
      left: 0;
      width: 100%;
      height: 100%;
      background: rgba(0, 0, 0, 0.8);
      display: flex;
      justify-content: center;
      align-items: center;
      z-index: 999999;
      font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
    `;

    const modal = document.createElement('div');
    modal.style.cssText = `
      background: white;
      padding: 30px;
      border-radius: 10px;
      max-width: 400px;
      text-align: center;
      box-shadow: 0 10px 30px rgba(0, 0, 0, 0.3);
    `;

    const icon = type === 'danger' ? '🛑' : '⚠️';
    const color = type === 'danger' ? '#f44336' : '#ff9800';

    modal.innerHTML = `
      <div style="font-size: 48px; margin-bottom: 16px;">${icon}</div>
      <h2 style="color: #333; margin-bottom: 12px;">${title}</h2>
      <p style="color: #666; margin-bottom: 24px; line-height: 1.5;">${message}</p>
      <div style="display: flex; gap: 12px; justify-content: center;">
        ${onContinue ? `
          <button id="ghost-continue-btn" style="
            background: ${color};
            color: white;
            border: none;
            padding: 12px 24px;
            border-radius: 6px;
            cursor: pointer;
            font-weight: 500;
          ">Continue</button>
        ` : ''}
        <button id="ghost-cancel-btn" style="
          background: #666;
          color: white;
          border: none;
          padding: 12px 24px;
          border-radius: 6px;
          cursor: pointer;
          font-weight: 500;
        ">Cancel</button>
      </div>
    `;

    overlay.appendChild(modal);

    // Add event listeners
    if (onContinue) {
      modal.querySelector('#ghost-continue-btn').addEventListener('click', onContinue);
    }
    modal.querySelector('#ghost-cancel-btn').addEventListener('click', () => {
      this.removeWarningOverlay();
    });

    return overlay;
  }

  removeWarningOverlay() {
    const overlay = document.getElementById('ghost-antivirus-warning-overlay');
    if (overlay) {
      overlay.remove();
    }
  }

  displayScanResult(scanResult) {
    // Create a subtle notification for scan results
    if (scanResult.isMalicious || scanResult.isPhishing) {
      const notification = document.createElement('div');
      notification.style.cssText = `
        position: fixed;
        top: 20px;
        right: 20px;
        background: #f44336;
        color: white;
        padding: 12px 16px;
        border-radius: 6px;
        font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
        font-size: 14px;
        box-shadow: 0 4px 12px rgba(0, 0, 0, 0.3);
        z-index: 999998;
        max-width: 300px;
      `;
      notification.innerHTML = `
        <div style="display: flex; align-items: center; gap: 8px;">
          <span>🛑</span>
          <div>
            <strong>Warning:</strong> This page contains security threats.
            <div style="font-size: 12px; margin-top: 4px; opacity: 0.9;">
              ${scanResult.recommendations.join(' ')}
            </div>
          </div>
        </div>
      `;
      
      document.body.appendChild(notification);
      
      // Auto-remove after 10 seconds
      setTimeout(() => {
        if (notification.parentNode) {
          notification.remove();
        }
      }, 10000);
    }
  }
}

// Initialize content script
new GhostAntivirusContent();