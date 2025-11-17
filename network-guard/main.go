package main

import (
	"context"
	"encoding/json"
	"fmt"
	"log"
	"net/http"
	"os"
	"os/signal"
	"syscall"
	"time"

	"github.com/gin-gonic/gin"
)

// Config holds the network guard configuration
type Config struct {
	Port        int    `yaml:"port" json:"port"`
	LogLevel    string `yaml:"log_level" json:"log_level"`
	EnableVPN   bool   `yaml:"enable_vpn" json:"enable_vpn"`
	VPNServer   string `yaml:"vpn_server" json:"vpn_server"`
	VPNPort     int    `yaml:"vpn_port" json:"vpn_port"`
}

// FirewallRule represents a firewall rule
type FirewallRule struct {
	ID          string `json:"id"`
	Name        string `json:"name"`
	Action      string `json:"action"` // allow, deny, block
	SourceIP    string `json:"source_ip"`
	DestIP      string `json:"dest_ip"`
	SourcePort  int    `json:"source_port"`
	DestPort    int    `json:"dest_port"`
	Protocol    string `json:"protocol"` // tcp, udp, icmp
	Enabled     bool   `json:"enabled"`
	Priority    int    `json:"priority"`
	CreatedAt   time.Time `json:"created_at"`
	UpdatedAt   time.Time `json:"updated_at"`
}

// NetworkConnection represents an active network connection
type NetworkConnection struct {
	ID         string    `json:"id"`
	ProcessID  int       `json:"process_id"`
	ProcessName string   `json:"process_name"`
	LocalIP    string    `json:"local_ip"`
	LocalPort  int       `json:"local_port"`
	RemoteIP   string    `json:"remote_ip"`
	RemotePort int       `json:"remote_port"`
	Protocol   string    `json:"protocol"`
	State      string    `json:"state"`
	BytesSent  int64     `json:"bytes_sent"`
	BytesRecv  int64     `json:"bytes_recv"`
	StartTime  time.Time `json:"start_time"`
	Threat     bool      `json:"threat"`
	ThreatType string    `json:"threat_type,omitempty"`
}

// NetworkStats represents network statistics
type NetworkStats struct {
	TotalConnections int   `json:"total_connections"`
	ActiveConnections int  `json:"active_connections"`
	BlockedConnections int `json:"blocked_connections"`
	BytesTransferred  int64 `json:"bytes_transferred"`
	PacketsTransferred int64 `json:"packets_transferred"`
	ThreatsBlocked    int   `json:"threats_blocked"`
	LastUpdate        time.Time `json:"last_update"`
}

// VPNConnection represents a VPN connection
type VPNConnection struct {
	ID          string    `json:"id"`
	Server      string    `json:"server"`
	Status      string    `json:"status"` // connected, disconnected, connecting
	Protocol    string    `json:"protocol"`
	IP          string    `json:"ip"`
	Location    string    `json:"location"`
	ConnectedAt time.Time `json:"connected_at"`
	BytesIn     int64     `json:"bytes_in"`
	BytesOut    int64     `json:"bytes_out"`
}

// NetworkGuard is the main network guard structure
type NetworkGuard struct {
	config     *Config
	firewall   *FirewallManager
	monitor    *NetworkMonitor
	vpn        *VPNManager
	stats      *NetworkStats
	connections []NetworkConnection
}

// FirewallManager manages firewall rules
type FirewallManager struct {
	rules []FirewallRule
}

// NetworkMonitor monitors network connections
type NetworkMonitor struct {
	connections []NetworkConnection
}

// VPNManager manages VPN connections
type VPNManager struct {
	connections []VPNConnection
	config     *Config
}

// NewNetworkGuard creates a new network guard instance
func NewNetworkGuard() *NetworkGuard {
	config := &Config{
		Port:      8080,
		LogLevel:  "info",
		EnableVPN: true,
		VPNServer: "vpn.ghostantivirus.com",
		VPNPort:   1194,
	}

	return &NetworkGuard{
		config:  config,
		firewall: &FirewallManager{
			rules: getDefaultFirewallRules(),
		},
		monitor: &NetworkMonitor{
			connections: []NetworkConnection{},
		},
		vpn: &VPNManager{
			connections: []VPNConnection{},
			config:     config,
		},
		stats: &NetworkStats{
			LastUpdate: time.Now(),
		},
		connections: []NetworkConnection{},
	}
}

// getDefaultFirewallRules returns default firewall rules
func getDefaultFirewallRules() []FirewallRule {
	now := time.Now()
	return []FirewallRule{
		{
			ID:         "rule-001",
			Name:       "Allow HTTP",
			Action:     "allow",
			SourceIP:   "0.0.0.0",
			DestIP:     "0.0.0.0",
			DestPort:   80,
			Protocol:   "tcp",
			Enabled:    true,
			Priority:   100,
			CreatedAt:  now,
			UpdatedAt:  now,
		},
		{
			ID:         "rule-002",
			Name:       "Allow HTTPS",
			Action:     "allow",
			SourceIP:   "0.0.0.0",
			DestIP:     "0.0.0.0",
			DestPort:   443,
			Protocol:   "tcp",
			Enabled:    true,
			Priority:   100,
			CreatedAt:  now,
			UpdatedAt:  now,
		},
		{
			ID:         "rule-003",
			Name:       "Block Known Malicious IPs",
			Action:     "block",
			SourceIP:   "192.168.1.100",
			DestIP:     "0.0.0.0",
			Protocol:   "all",
			Enabled:    true,
			Priority:   10,
			CreatedAt:  now,
			UpdatedAt:  now,
		},
	}
}

// Start starts the network guard service
func (ng *NetworkGuard) Start() error {
	log.Printf("Starting GhostAntivirus Network Guard on port %d", ng.config.Port)
	
	// Start network monitoring
	go ng.startNetworkMonitoring()
	
	// Setup HTTP server
	router := gin.Default()
	ng.setupRoutes(router)
	
	server := &http.Server{
		Addr:    fmt.Sprintf(":%d", ng.config.Port),
		Handler: router,
	}
	
	// Start server in goroutine
	go func() {
		if err := server.ListenAndServe(); err != nil && err != http.ErrServerClosed {
			log.Fatalf("Failed to start server: %v", err)
		}
	}()
	
	// Wait for interrupt signal to gracefully shutdown
	quit := make(chan os.Signal, 1)
	signal.Notify(quit, syscall.SIGINT, syscall.SIGTERM)
	<-quit
	
	log.Println("Shutting down Network Guard...")
	
	ctx, cancel := context.WithTimeout(context.Background(), 5*time.Second)
	defer cancel()
	
	if err := server.Shutdown(ctx); err != nil {
		log.Fatal("Server forced to shutdown:", err)
	}
	
	log.Println("Network Guard exited")
	return nil
}

// startNetworkMonitoring starts monitoring network connections
func (ng *NetworkGuard) startNetworkMonitoring() {
	ticker := time.NewTicker(5 * time.Second)
	defer ticker.Stop()
	
	for {
		select {
		case <-ticker.C:
			ng.scanNetworkConnections()
			ng.updateStats()
		}
	}
}

// scanNetworkConnections scans for active network connections
func (ng *NetworkGuard) scanNetworkConnections() {
	// Simulate network connection scanning
	connections := []NetworkConnection{
		{
			ID:          "conn-001",
			ProcessID:   1234,
			ProcessName: "chrome.exe",
			LocalIP:     "192.168.1.100",
			LocalPort:   52345,
			RemoteIP:    "142.250.191.78",
			RemotePort:  443,
			Protocol:    "tcp",
			State:       "ESTABLISHED",
			BytesSent:   1024,
			BytesRecv:   2048,
			StartTime:   time.Now().Add(-10 * time.Minute),
			Threat:      false,
		},
		{
			ID:          "conn-002",
			ProcessID:   5678,
			ProcessName: "firefox.exe",
			LocalIP:     "192.168.1.100",
			LocalPort:   54321,
			RemoteIP:    "157.240.229.35",
			RemotePort:  443,
			Protocol:    "tcp",
			State:       "ESTABLISHED",
			BytesSent:   512,
			BytesRecv:   1024,
			StartTime:   time.Now().Add(-5 * time.Minute),
			Threat:      false,
		},
		{
			ID:          "conn-003",
			ProcessID:   9999,
			ProcessName: "malware.exe",
			LocalIP:     "192.168.1.100",
			LocalPort:   55555,
			RemoteIP:    "192.168.1.200",
			RemotePort:  8080,
			Protocol:    "tcp",
			State:       "ESTABLISHED",
			BytesSent:   4096,
			BytesRecv:   1024,
			StartTime:   time.Now().Add(-2 * time.Minute),
			Threat:      true,
			ThreatType:  "C2 Communication",
		},
	}
	
	ng.connections = connections
	ng.monitor.connections = connections
}

// updateStats updates network statistics
func (ng *NetworkGuard) updateStats() {
	threatCount := 0
	activeCount := 0
	blockedCount := 0
	
	for _, conn := range ng.connections {
		if conn.Threat {
			threatCount++
		}
		if conn.State == "ESTABLISHED" {
			activeCount++
		}
	}
	
	ng.stats = &NetworkStats{
		TotalConnections:   len(ng.connections),
		ActiveConnections:  activeCount,
		BlockedConnections: blockedCount,
		BytesTransferred:   1024 * 1024, // 1MB
		PacketsTransferred: 1024,
		ThreatsBlocked:     threatCount,
		LastUpdate:         time.Now(),
	}
}

// setupRoutes sets up HTTP routes
func (ng *NetworkGuard) setupRoutes(router *gin.Engine) {
	// CORS middleware
	router.Use(func(c *gin.Context) {
		c.Header("Access-Control-Allow-Origin", "*")
		c.Header("Access-Control-Allow-Methods", "GET, POST, PUT, DELETE, OPTIONS")
		c.Header("Access-Control-Allow-Headers", "Content-Type, Authorization")
		
		if c.Request.Method == "OPTIONS" {
			c.AbortWithStatus(204)
			return
		}
		
		c.Next()
	})
	
	api := router.Group("/api/v1")
	{
		// Firewall routes
		api.GET("/firewall/rules", ng.getFirewallRules)
		api.POST("/firewall/rules", ng.createFirewallRule)
		api.PUT("/firewall/rules/:id", ng.updateFirewallRule)
		api.DELETE("/firewall/rules/:id", ng.deleteFirewallRule)
		
		// Network monitoring routes
		api.GET("/network/connections", ng.getNetworkConnections)
		api.GET("/network/stats", ng.getNetworkStats)
		api.POST("/network/scan", ng.scanNetwork)
		
		// VPN routes
		api.GET("/vpn/status", ng.getVPNStatus)
		api.POST("/vpn/connect", ng.connectVPN)
		api.POST("/vpn/disconnect", ng.disconnectVPN)
		api.GET("/vpn/connections", ng.getVPNConnections)
		
		// Health check
		api.GET("/health", ng.healthCheck)
	}
}

// getFirewallRules returns firewall rules
func (ng *NetworkGuard) getFirewallRules(c *gin.Context) {
	c.JSON(http.StatusOK, gin.H{
		"success": true,
		"data":    ng.firewall.rules,
	})
}

// createFirewallRule creates a new firewall rule
func (ng *NetworkGuard) createFirewallRule(c *gin.Context) {
	var rule FirewallRule
	if err := c.ShouldBindJSON(&rule); err != nil {
		c.JSON(http.StatusBadRequest, gin.H{
			"success": false,
			"error":   err.Error(),
		})
		return
	}
	
	rule.ID = fmt.Sprintf("rule-%d", len(ng.firewall.rules)+1)
	rule.CreatedAt = time.Now()
	rule.UpdatedAt = time.Now()
	
	ng.firewall.rules = append(ng.firewall.rules, rule)
	
	c.JSON(http.StatusCreated, gin.H{
		"success": true,
		"data":    rule,
	})
}

// updateFirewallRule updates a firewall rule
func (ng *NetworkGuard) updateFirewallRule(c *gin.Context) {
	id := c.Param("id")
	var updatedRule FirewallRule
	
	if err := c.ShouldBindJSON(&updatedRule); err != nil {
		c.JSON(http.StatusBadRequest, gin.H{
			"success": false,
			"error":   err.Error(),
		})
		return
	}
	
	for i, rule := range ng.firewall.rules {
		if rule.ID == id {
			updatedRule.ID = id
			updatedRule.CreatedAt = rule.CreatedAt
			updatedRule.UpdatedAt = time.Now()
			ng.firewall.rules[i] = updatedRule
			
			c.JSON(http.StatusOK, gin.H{
				"success": true,
				"data":    updatedRule,
			})
			return
		}
	}
	
	c.JSON(http.StatusNotFound, gin.H{
		"success": false,
		"error":   "Rule not found",
	})
}

// deleteFirewallRule deletes a firewall rule
func (ng *NetworkGuard) deleteFirewallRule(c *gin.Context) {
	id := c.Param("id")
	
	for i, rule := range ng.firewall.rules {
		if rule.ID == id {
			ng.firewall.rules = append(ng.firewall.rules[:i], ng.firewall.rules[i+1:]...)
			
			c.JSON(http.StatusOK, gin.H{
				"success": true,
				"message": "Rule deleted successfully",
			})
			return
		}
	}
	
	c.JSON(http.StatusNotFound, gin.H{
		"success": false,
		"error":   "Rule not found",
	})
}

// getNetworkConnections returns active network connections
func (ng *NetworkGuard) getNetworkConnections(c *gin.Context) {
	c.JSON(http.StatusOK, gin.H{
		"success": true,
		"data":    ng.connections,
	})
}

// getNetworkStats returns network statistics
func (ng *NetworkGuard) getNetworkStats(c *gin.Context) {
	c.JSON(http.StatusOK, gin.H{
		"success": true,
		"data":    ng.stats,
	})
}

// scanNetwork triggers a network scan
func (ng *NetworkGuard) scanNetwork(c *gin.Context) {
	ng.scanNetworkConnections()
	
	c.JSON(http.StatusOK, gin.H{
		"success": true,
		"message": "Network scan completed",
		"data":    ng.connections,
	})
}

// getVPNStatus returns VPN status
func (ng *NetworkGuard) getVPNStatus(c *gin.Context) {
	status := "disconnected"
	if len(ng.vpn.connections) > 0 {
		for _, conn := range ng.vpn.connections {
			if conn.Status == "connected" {
				status = "connected"
				break
			}
		}
	}
	
	c.JSON(http.StatusOK, gin.H{
		"success": true,
		"data": gin.H{
			"status":      status,
			"enabled":     ng.config.EnableVPN,
			"server":      ng.config.VPNServer,
			"connections": ng.vpn.connections,
		},
	})
}

// connectVPN connects to VPN
func (ng *NetworkGuard) connectVPN(c *gin.Context) {
	connection := VPNConnection{
		ID:          fmt.Sprintf("vpn-%d", len(ng.vpn.connections)+1),
		Server:      ng.config.VPNServer,
		Status:      "connected",
		Protocol:    "wireguard",
		IP:          "10.0.0.2",
		Location:    "US West",
		ConnectedAt: time.Now(),
		BytesIn:     0,
		BytesOut:    0,
	}
	
	ng.vpn.connections = append(ng.vpn.connections, connection)
	
	c.JSON(http.StatusOK, gin.H{
		"success": true,
		"data":    connection,
	})
}

// disconnectVPN disconnects from VPN
func (ng *NetworkGuard) disconnectVPN(c *gin.Context) {
	if len(ng.vpn.connections) > 0 {
		ng.vpn.connections = []VPNConnection{}
	}
	
	c.JSON(http.StatusOK, gin.H{
		"success": true,
		"message": "VPN disconnected",
	})
}

// getVPNConnections returns VPN connections
func (ng *NetworkGuard) getVPNConnections(c *gin.Context) {
	c.JSON(http.StatusOK, gin.H{
		"success": true,
		"data":    ng.vpn.connections,
	})
}

// healthCheck performs health check
func (ng *NetworkGuard) healthCheck(c *gin.Context) {
	c.JSON(http.StatusOK, gin.H{
		"success": true,
		"data": gin.H{
			"status":    "healthy",
			"timestamp": time.Now(),
			"version":   "3.0.0",
			"uptime":    "0h 0m 0s",
		},
	})
}

func main() {
	networkGuard := NewNetworkGuard()
	
	if err := networkGuard.Start(); err != nil {
		log.Fatalf("Failed to start Network Guard: %v", err)
	}
}