// GhostAntivirus Browser Extension - Popup Script
class GhostAntivirusPopup {
  constructor() {
    this.initializeElements();
    this.attachEventListeners();
    this.loadStats();
    this.scanCurrentPage();
    this.loadSettings();
  }

  initializeElements() {
    // Status elements
    this.statusIndicator = document.getElementById('statusIndicator');
    this.protectionToggle = document.getElementById('protectionToggle');
    
    // Stats elements
    this.threatsBlockedEl = document.getElementById('threatsBlocked');
    this.scansPerformedEl = document.getElementById('scansPerformed');
    this.adsBlockedEl = document.getElementById('adsBlocked');
    
    // Page scan elements
    this.currentPageUrlEl = document.getElementById('currentPageUrl');
    this.pageStatusEl = document.getElementById('pageStatus');
    this.scanResultEl = document.getElementById('scanResult');
    
    // Feature toggles
    this.antiPhishingToggle = document.getElementById('antiPhishingToggle');
    this.adBlockingToggle = document.getElementById('adBlockingToggle');
    this.trackerBlockingToggle = document.getElementById('trackerBlockingToggle');
    this.cookieProtectionToggle = document.getElementById('cookieProtectionToggle');
    
    // Buttons
    this.scanCurrentPageBtn = document.getElementById('scanCurrentPageBtn');
    this.reportSiteBtn = document.getElementById('reportSiteBtn');
    this.settingsBtn = document.getElementById('settingsBtn');
    this.dashboardBtn = document.getElementById('dashboardBtn');
  }

  attachEventListeners() {
    // Protection toggle
    this.protectionToggle.addEventListener('change', () => {
      this.updateProtectionStatus();
    });

    // Feature toggles
    this.antiPhishingToggle.addEventListener('change', () => {
      this.updateFeatureSetting('antiPhishing', this.antiPhishingToggle.checked);
    });

    this.adBlockingToggle.addEventListener('change', () => {
      this.updateFeatureSetting('adBlocking', this.adBlockingToggle.checked);
    });

    this.trackerBlockingToggle.addEventListener('change', () => {
      this.updateFeatureSetting('trackerBlocking', this.trackerBlockingToggle.checked);
    });

    this.cookieProtectionToggle.addEventListener('change', () => {
      this.updateFeatureSetting('cookieProtection', this.cookieProtectionToggle.checked);
    });

    // Action buttons
    this.scanCurrentPageBtn.addEventListener('click', () => {
      this.scanCurrentPage(true);
    });

    this.reportSiteBtn.addEventListener('click', () => {
      this.reportCurrentSite();
    });

    this.settingsBtn.addEventListener('click', () => {
      chrome.tabs.create({ url: 'src/settings.html' });
    });

    this.dashboardBtn.addEventListener('click', () => {
      chrome.tabs.create({ url: 'https://localhost:3000/dashboard' });
    });
  }

  async loadStats() {
    try {
      const response = await this.sendMessage({ action: 'getStats' });
      if (response.success) {
        this.updateStatsDisplay(response.data);
      }
    } catch (error) {
      console.error('Failed to load stats:', error);
    }
  }

  updateStatsDisplay(stats) {
    // Animate number counting
    this.animateNumber(this.threatsBlockedEl, stats.threatsBlocked || 0);
    this.animateNumber(this.scansPerformedEl, stats.scansPerformed || 0);
    this.animateNumber(this.adsBlockedEl, stats.adsBlocked || 0);
  }

  animateNumber(element, target) {
    const duration = 1000;
    const start = parseInt(element.textContent) || 0;
    const increment = (target - start) / (duration / 16);
    let current = start;

    const timer = setInterval(() => {
      current += increment;
      if ((increment > 0 && current >= target) || (increment < 0 && current <= target)) {
        element.textContent = target.toLocaleString();
        clearInterval(timer);
      } else {
        element.textContent = Math.floor(current).toLocaleString();
      }
    }, 16);
  }

  async loadSettings() {
    try {
      const response = await this.sendMessage({ action: 'getSettings' });
      if (response.success) {
        this.updateSettingsDisplay(response.data);
      }
    } catch (error) {
      console.error('Failed to load settings:', error);
    }
  }

  updateSettingsDisplay(settings) {
    this.protectionToggle.checked = settings.enabled !== false;
    this.antiPhishingToggle.checked = settings.antiPhishing !== false;
    this.adBlockingToggle.checked = settings.adBlocking !== false;
    this.trackerBlockingToggle.checked = settings.trackerBlocking !== false;
    this.cookieProtectionToggle.checked = settings.cookieProtection !== false;

    this.updateStatusIndicator(settings.enabled !== false);
    this.updateFeatureStatuses();
  }

  updateStatusIndicator(enabled) {
    const statusDot = this.statusIndicator.querySelector('.status-dot');
    const statusText = this.statusIndicator.querySelector('.status-text');

    if (enabled) {
      statusDot.className = 'status-dot protected';
      statusText.textContent = 'Protected';
    } else {
      statusDot.className = 'status-dot disabled';
      statusText.textContent = 'Disabled';
    }
  }

  updateFeatureStatuses() {
    const features = [
      { toggle: this.antiPhishingToggle, statusId: 'antiPhishingStatus' },
      { toggle: this.adBlockingToggle, statusId: 'adBlockingStatus' },
      { toggle: this.trackerBlockingToggle, statusId: 'trackerBlockingStatus' },
      { toggle: this.cookieProtectionToggle, statusId: 'cookieProtectionStatus' }
    ];

    features.forEach(feature => {
      const statusEl = document.getElementById(feature.statusId);
      if (feature.toggle.checked) {
        statusEl.textContent = 'Active';
        statusEl.className = 'feature-status active';
      } else {
        statusEl.textContent = 'Inactive';
        statusEl.className = 'feature-status inactive';
      }
    });
  }

  async scanCurrentPage(force = false) {
    try {
      // Get current tab URL
      const [tab] = await chrome.tabs.query({ active: true, currentWindow: true });
      
      if (!tab || !tab.url) {
        this.updatePageScanResult('error', 'Unable to determine current page');
        return;
      }

      // Display URL
      this.currentPageUrlEl.textContent = this.truncateUrl(tab.url);
      
      // Show scanning status
      if (force) {
        this.setPageStatus('scanning', 'Scanning...');
      }

      // Send scan request
      const response = await this.sendMessage({ 
        action: 'scanUrl', 
        url: tab.url 
      });

      if (response.success) {
        this.displayScanResult(response.data);
      } else {
        this.updatePageScanResult('error', 'Scan failed: ' + response.error);
      }
    } catch (error) {
      this.updatePageScanResult('error', 'Scan failed: ' + error.message);
    }
  }

  displayScanResult(scanResult) {
    const { isMalicious, isPhishing, isTracker, reputation, recommendations } = scanResult;

    if (isMalicious || isPhishing) {
      this.setPageStatus('danger', 'Dangerous');
      this.updatePageScanResult('danger', `
        <div class="scan-result-content">
          <div class="result-icon danger">🛑</div>
          <div class="result-title">Dangerous Website Detected</div>
          <div class="result-description">
            This site is known for malicious activity and may harm your computer.
          </div>
          <div class="recommendations">
            <strong>Recommendations:</strong>
            <ul>
              ${recommendations.map(rec => `<li>${rec}</li>`).join('')}
            </ul>
          </div>
        </div>
      `);
    } else if (isTracker) {
      this.setPageStatus('warning', 'Tracker');
      this.updatePageScanResult('warning', `
        <div class="scan-result-content">
          <div class="result-icon warning">⚠️</div>
          <div class="result-title">Tracking Website</div>
          <div class="result-description">
            This website may track your online activities for advertising or analytics purposes.
          </div>
        </div>
      `);
    } else {
      this.setPageStatus('safe', 'Safe');
      this.updatePageScanResult('safe', `
        <div class="scan-result-content">
          <div class="result-icon safe">✅</div>
          <div class="result-title">Website is Safe</div>
          <div class="result-description">
            No threats detected on this website. Continue browsing safely.
          </div>
        </div>
      `);
    }
  }

  setPageStatus(status, text) {
    const scanIndicator = this.pageStatusEl.querySelector('.scan-indicator');
    const scanText = this.pageStatusEl.querySelector('.scan-text');

    scanIndicator.className = `scan-indicator ${status}`;
    scanText.textContent = text;
  }

  updatePageScanResult(type, content) {
    this.scanResultEl.className = `scan-result ${type}`;
    this.scanResultEl.innerHTML = content;
  }

  truncateUrl(url) {
    try {
      const urlObj = new URL(url);
      return urlObj.hostname;
    } catch (error) {
      return url;
    }
  }

  async updateProtectionStatus() {
    const enabled = this.protectionToggle.checked;
    
    try {
      const response = await this.sendMessage({ 
        action: 'updateSettings',
        settings: { enabled }
      });

      if (response.success) {
        this.updateStatusIndicator(enabled);
      } else {
        this.protectionToggle.checked = !enabled; // Revert on failure
      }
    } catch (error) {
      this.protectionToggle.checked = !enabled; // Revert on failure
      console.error('Failed to update protection status:', error);
    }
  }

  async updateFeatureSetting(feature, enabled) {
    try {
      const response = await this.sendMessage({ 
        action: 'updateSettings',
        settings: { [feature]: enabled }
      });

      if (response.success) {
        this.updateFeatureStatuses();
      } else {
        // Revert on failure
        const toggle = this[`${feature}Toggle`];
        if (toggle) toggle.checked = !enabled;
      }
    } catch (error) {
      // Revert on failure
      const toggle = this[`${feature}Toggle`];
      if (toggle) toggle.checked = !enabled;
      console.error(`Failed to update ${feature} setting:`, error);
    }
  }

  async reportCurrentSite() {
    try {
      const [tab] = await chrome.tabs.query({ active: true, currentWindow: true });
      
      if (!tab || !tab.url) {
        alert('Unable to determine current site');
        return;
      }

      // In a real implementation, this would send the report to GhostAntivirus servers
      console.log('Reporting site:', tab.url);
      
      // Show confirmation
      const confirmation = confirm(`Report ${this.truncateUrl(tab.url)} as a malicious site?`);
      
      if (confirmation) {
        // Send report (mock implementation)
        alert('Thank you for your report. Our team will investigate this site.');
      }
    } catch (error) {
      console.error('Failed to report site:', error);
      alert('Failed to report site. Please try again.');
    }
  }

  sendMessage(request) {
    return new Promise((resolve) => {
      chrome.runtime.sendMessage(request, (response) => {
        if (chrome.runtime.lastError) {
          resolve({ success: false, error: chrome.runtime.lastError.message });
        } else {
          resolve(response);
        }
      });
    });
  }
}

// Initialize popup when DOM is loaded
document.addEventListener('DOMContentLoaded', () => {
  new GhostAntivirusPopup();
});