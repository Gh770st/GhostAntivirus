// GhostAntivirus Browser Extension - Background Service Worker
class GhostAntivirusBackground {
  constructor() {
    this.initialize();
  }

  initialize() {
    console.log('GhostAntivirus Background Service Worker initialized');
    
    // Initialize default settings
    this.initializeSettings();
    
    // Set up event listeners
    this.setupEventListeners();
    
    // Initialize protection modules
    this.initializeProtection();
  }

  initializeSettings() {
    const defaultSettings = {
      enabled: true,
      antiPhishing: true,
      maliciousSitesBlocking: true,
      safeSearch: false,
      adBlocking: true,
      trackerBlocking: true,
      cookieProtection: true,
      notifications: true,
      scanDownloads: true,
      protectionLevel: 'medium', // low, medium, high
      customBlocklist: [],
      customAllowlist: []
    };

    chrome.storage.sync.get(['settings'], (result) => {
      if (!result.settings) {
        chrome.storage.sync.set({ settings: defaultSettings });
      }
    });
  }

  setupEventListeners() {
    // Web request blocking and monitoring
    chrome.webRequest.onBeforeRequest.addListener(
      this.handleWebRequest.bind(this),
      { urls: ['<all_urls>'] },
      ['blocking']
    );

    chrome.webRequest.onHeadersReceived.addListener(
      this.handleResponseHeaders.bind(this),
      { urls: ['<all_urls>'] },
      ['blocking']
    );

    // Tab events
    chrome.tabs.onUpdated.addListener(this.handleTabUpdate.bind(this));
    chrome.tabs.onCreated.addListener(this.handleTabCreated.bind(this));

    // Download events
    chrome.downloads.onDeterminingFilename.addListener(this.handleDownload.bind(this));

    // Storage events
    chrome.storage.onChanged.addListener(this.handleStorageChange.bind(this));

    // Extension installation/update
    chrome.runtime.onInstalled.addListener(this.handleInstall.bind(this));

    // Messages from popup and content scripts
    chrome.runtime.onMessage.addListener(this.handleMessage.bind(this));
  }

  initializeProtection() {
    // Initialize threat intelligence databases
    this.threatDatabase = {
      maliciousDomains: new Set(),
      phishingSites: new Set(),
      trackingDomains: new Set(),
      adDomains: new Set()
    };

    // Load threat databases
    this.loadThreatDatabases();

    // Initialize real-time protection
    this.protectionStats = {
      threatsBlocked: 0,
      phishingAttempts: 0,
      adsBlocked: 0,
      trackersBlocked: 0,
      scansPerformed: 0
    };
  }

  async loadThreatDatabases() {
    try {
      // In a real implementation, this would fetch from GhostAntivirus servers
      // For now, we'll use some example threats
      const sampleThreats = {
        maliciousDomains: [
          'malware-site.com',
          'phishing-attempt.net',
          'suspicious-domain.org'
        ],
        phishingSites: [
          'fake-paypal.com',
          'phishing-bank.net',
          'scam-website.org'
        ],
        trackingDomains: [
          'google-analytics.com',
          'facebook.com/tr',
          'doubleclick.net'
        ],
        adDomains: [
          'googleads.g.doubleclick.net',
          'googleadservices.com',
          'googlesyndication.com'
        ]
      };

      this.threatDatabase.maliciousDomains = new Set(sampleThreats.maliciousDomains);
      this.threatDatabase.phishingSites = new Set(sampleThreats.phishingSites);
      this.threatDatabase.trackingDomains = new Set(sampleThreats.trackingDomains);
      this.threatDatabase.adDomains = new Set(sampleThreats.adDomains);

      console.log('Threat databases loaded successfully');
    } catch (error) {
      console.error('Failed to load threat databases:', error);
    }
  }

  async handleWebRequest(details) {
    const { url, type, tabId } = details;
    
    // Skip if extension is disabled
    const settings = await this.getSettings();
    if (!settings.enabled) {
      return { cancel: false };
    }

    const domain = this.extractDomain(url);
    let shouldBlock = false;
    let blockReason = '';

    // Check against malicious domains
    if (this.threatDatabase.maliciousDomains.has(domain)) {
      shouldBlock = true;
      blockReason = 'Malicious domain detected';
      this.protectionStats.threatsBlocked++;
    }
    // Check against phishing sites
    else if (settings.antiPhishing && this.threatDatabase.phishingSites.has(domain)) {
      shouldBlock = true;
      blockReason = 'Phishing site detected';
      this.protectionStats.phishingAttempts++;
    }
    // Check against ad domains
    else if (settings.adBlocking && this.threatDatabase.adDomains.has(domain)) {
      if (type === 'script' || type === 'sub_frame') {
        shouldBlock = true;
        blockReason = 'Advertisement blocked';
        this.protectionStats.adsBlocked++;
      }
    }
    // Check against tracking domains
    else if (settings.trackerBlocking && this.threatDatabase.trackingDomains.has(domain)) {
      shouldBlock = true;
      blockReason = 'Tracker blocked';
      this.protectionStats.trackersBlocked++;
    }

    // Check custom blocklist
    if (settings.customBlocklist.includes(domain)) {
      shouldBlock = true;
      blockReason = 'Custom blocklist';
    }

    if (shouldBlock) {
      console.log(`Blocking ${url}: ${blockReason}`);
      
      // Send notification
      if (settings.notifications) {
        this.sendNotification('Threat Blocked', `${blockReason}: ${domain}`);
      }

      // Update blocked page
      this.updateBlockedPage(tabId, domain, blockReason);

      return { cancel: true };
    }

    return { cancel: false };
  }

  async handleResponseHeaders(details) {
    const settings = await this.getSettings();
    if (!settings.cookieProtection) {
      return {};
    }

    const headers = details.responseHeaders;
    const modifiedHeaders = headers.filter(header => {
      // Remove suspicious cookies
      if (header.name.toLowerCase() === 'set-cookie') {
        const cookieValue = header.value.toLowerCase();
        return !cookieValue.includes('session') && !cookieValue.includes('token');
      }
      return true;
    });

    return { responseHeaders: modifiedHeaders };
  }

  handleTabUpdate(tabId, changeInfo, tab) {
    if (changeInfo.status === 'complete' && tab.url) {
      this.scanPage(tabId, tab.url);
    }
  }

  handleTabCreated(tab) {
    // New tab created
    console.log('New tab created:', tab.id);
  }

  async handleDownload(downloadItem) {
    const settings = await this.getSettings();
    if (!settings.scanDownloads) {
      return {};
    }

    const { url, filename } = downloadItem;
    
    // Simple download scanning based on file extension and URL
    const dangerousExtensions = ['.exe', '.scr', '.bat', '.com', '.pif', '.vbs', '.js'];
    const isDangerous = dangerousExtensions.some(ext => filename.toLowerCase().endsWith(ext));
    
    if (isDangerous || this.threatDatabase.maliciousDomains.has(this.extractDomain(url))) {
      this.sendNotification('Download Warning', `Dangerous download detected: ${filename}`);
      
      // In a real implementation, you might want to block the download or show a warning
      console.warn('Dangerous download detected:', filename, url);
    }

    return {};
  }

  handleStorageChange(changes, namespace) {
    if (changes.settings) {
      console.log('Settings updated:', changes.settings.newValue);
    }
  }

  handleInstall(details) {
    if (details.reason === 'install') {
      console.log('GhostAntivirus extension installed');
      
      // Open welcome page
      chrome.tabs.create({
        url: chrome.runtime.getURL('src/welcome.html')
      });
    } else if (details.reason === 'update') {
      console.log('GhostAntivirus extension updated');
    }
  }

  async handleMessage(request, sender, sendResponse) {
    try {
      switch (request.action) {
        case 'getStats':
          sendResponse({ success: true, data: this.protectionStats });
          break;
          
        case 'getSettings':
          const settings = await this.getSettings();
          sendResponse({ success: true, data: settings });
          break;
          
        case 'updateSettings':
          await chrome.storage.sync.set({ settings: request.settings });
          sendResponse({ success: true });
          break;
          
        case 'scanUrl':
          const scanResult = await this.scanUrl(request.url);
          sendResponse({ success: true, data: scanResult });
          break;
          
        case 'resetStats':
          this.protectionStats = {
            threatsBlocked: 0,
            phishingAttempts: 0,
            adsBlocked: 0,
            trackersBlocked: 0,
            scansPerformed: 0
          };
          sendResponse({ success: true });
          break;
          
        default:
          sendResponse({ success: false, error: 'Unknown action' });
      }
    } catch (error) {
      sendResponse({ success: false, error: error.message });
    }
    
    return true; // Keep message channel open for async response
  }

  async getSettings() {
    return new Promise((resolve) => {
      chrome.storage.sync.get(['settings'], (result) => {
        resolve(result.settings || {});
      });
    });
  }

  extractDomain(url) {
    try {
      const urlObj = new URL(url);
      return urlObj.hostname.replace('www.', '');
    } catch (error) {
      return '';
    }
  }

  async scanUrl(url) {
    const domain = this.extractDomain(url);
    const scanResult = {
      url,
      domain,
      isMalicious: this.threatDatabase.maliciousDomains.has(domain),
      isPhishing: this.threatDatabase.phishingSites.has(domain),
      isTracker: this.threatDatabase.trackingDomains.has(domain),
      reputation: 'unknown', // In real implementation, this would be calculated
      recommendations: []
    };

    if (scanResult.isMalicious) {
      scanResult.recommendations.push('This domain is known for malicious activity');
      scanResult.reputation = 'dangerous';
    } else if (scanResult.isPhishing) {
      scanResult.recommendations.push('This is a known phishing site');
      scanResult.reputation = 'dangerous';
    } else if (scanResult.isTracker) {
      scanResult.recommendations.push('This domain tracks user activity');
      scanResult.reputation = 'suspicious';
    }

    this.protectionStats.scansPerformed++;
    return scanResult;
  }

  async scanPage(tabId, url) {
    const settings = await this.getSettings();
    if (!settings.enabled) return;

    const scanResult = await this.scanUrl(url);
    
    // Send scan results to content script
    chrome.tabs.sendMessage(tabId, {
      action: 'pageScanResult',
      data: scanResult
    });
  }

  sendNotification(title, message) {
    chrome.notifications.create({
      type: 'basic',
      iconUrl: chrome.runtime.getURL('icons/icon48.png'),
      title: `GhostAntivirus - ${title}`,
      message: message
    });
  }

  updateBlockedPage(tabId, domain, reason) {
    // Inject a custom blocked page
    chrome.scripting.executeScript({
      target: { tabId },
      func: (blockedDomain, blockReason) => {
        document.documentElement.innerHTML = `
          <html>
            <head>
              <title>Access Blocked - GhostAntivirus</title>
              <style>
                body {
                  font-family: Arial, sans-serif;
                  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
                  margin: 0;
                  padding: 0;
                  display: flex;
                  justify-content: center;
                  align-items: center;
                  min-height: 100vh;
                }
                .container {
                  background: white;
                  padding: 40px;
                  border-radius: 10px;
                  box-shadow: 0 10px 30px rgba(0,0,0,0.3);
                  text-align: center;
                  max-width: 500px;
                }
                .icon {
                  font-size: 72px;
                  color: #f44336;
                  margin-bottom: 20px;
                }
                h1 {
                  color: #333;
                  margin-bottom: 20px;
                }
                p {
                  color: #666;
                  line-height: 1.6;
                  margin-bottom: 30px;
                }
                .url {
                  background: #f5f5f5;
                  padding: 10px;
                  border-radius: 5px;
                  font-family: monospace;
                  word-break: break-all;
                  margin: 20px 0;
                }
                .reason {
                  background: #fff3cd;
                  color: #856404;
                  padding: 15px;
                  border-radius: 5px;
                  margin: 20px 0;
                }
                button {
                  background: #4caf50;
                  color: white;
                  border: none;
                  padding: 12px 30px;
                  border-radius: 5px;
                  cursor: pointer;
                  font-size: 16px;
                  margin: 5px;
                }
                button:hover {
                  background: #45a049;
                }
                button.danger {
                  background: #f44336;
                }
                button.danger:hover {
                  background: #da190b;
                }
              </style>
            </head>
            <body>
              <div class="container">
                <div class="icon">🛡️</div>
                <h1>Access Blocked</h1>
                <p>GhostAntivirus has blocked access to this website to protect your security and privacy.</p>
                
                <div class="url">${blockedDomain}</div>
                
                <div class="reason">
                  <strong>Reason:</strong> ${blockReason}
                </div>
                
                <p>This page may contain malicious content, phishing attempts, or other security threats that could harm your computer or compromise your personal information.</p>
                
                <button onclick="history.back()">Go Back</button>
                <button class="danger" onclick="window.close()">Close Tab</button>
              </div>
            </body>
          </html>
        `;
      },
      args: [domain, reason]
    });
  }
}

// Initialize the background service
const ghostAntivirus = new GhostAntivirusBackground();