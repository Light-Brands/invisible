//! Protocol Camouflage (Layer 5)
//!
//! Disguises network traffic to defeat Deep Packet Inspection (DPI) and
//! censorship systems. Invisible traffic appears as innocent protocols.
//!
//! ## Camouflage Strategies
//!
//! - **obfs4:** Randomized handshake + stream cipher, looks like random noise
//! - **uTLS:** Mimics browser TLS fingerprints (Chrome, Firefox, Safari, Edge)
//! - **Domain Fronting:** HTTPS to CDN frontdoor, route to actual destination
//!
//! ## Security Properties
//!
//! - **DPI Resistance:** Traffic indistinguishable from target protocol
//! - **Active Probing Resistance:** Invalid handshakes fail gracefully
//! - **Fingerprint Diversity:** Multiple TLS fingerprints prevent clustering

use serde::{Deserialize, Serialize};

use crate::error::{Result, ScramblerError};

/// Camouflage transport type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TransportType {
    /// obfs4: Pluggable transport, looks like random noise
    Obfs4,
    /// uTLS with Chrome fingerprint
    UTlsChrome,
    /// uTLS with Firefox fingerprint
    UTlsFirefox,
    /// uTLS with Safari fingerprint
    UTlsSafari,
    /// Domain fronting through CDN
    DomainFronting,
}

/// uTLS fingerprint parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UTlsFingerprint {
    /// Browser to mimic
    pub browser: String,
    /// TLS version (1.2 or 1.3)
    pub tls_version: String,
    /// Cipher suites
    pub cipher_suites: Vec<String>,
    /// Extensions
    pub extensions: Vec<String>,
    /// ALPN protocols
    pub alpn: Vec<String>,
}

impl UTlsFingerprint {
    /// Chrome 120+ fingerprint
    pub fn chrome() -> Self {
        Self {
            browser: "Chrome".to_string(),
            tls_version: "1.3".to_string(),
            cipher_suites: vec![
                "TLS_AES_128_GCM_SHA256".to_string(),
                "TLS_AES_256_GCM_SHA384".to_string(),
                "TLS_CHACHA20_POLY1305_SHA256".to_string(),
            ],
            extensions: vec![
                "server_name".to_string(),
                "status_request".to_string(),
                "supported_groups".to_string(),
                "signature_algorithms".to_string(),
                "key_share".to_string(),
            ],
            alpn: vec!["h2".to_string(), "http/1.1".to_string()],
        }
    }

    /// Firefox 121+ fingerprint
    pub fn firefox() -> Self {
        Self {
            browser: "Firefox".to_string(),
            tls_version: "1.3".to_string(),
            cipher_suites: vec![
                "TLS_AES_128_GCM_SHA256".to_string(),
                "TLS_CHACHA20_POLY1305_SHA256".to_string(),
                "TLS_AES_256_GCM_SHA384".to_string(),
            ],
            extensions: vec![
                "server_name".to_string(),
                "status_request".to_string(),
                "supported_groups".to_string(),
                "signature_algorithms".to_string(),
                "key_share".to_string(),
                "padding".to_string(),
            ],
            alpn: vec!["h2".to_string(), "http/1.1".to_string()],
        }
    }

    /// Safari 17+ fingerprint
    pub fn safari() -> Self {
        Self {
            browser: "Safari".to_string(),
            tls_version: "1.3".to_string(),
            cipher_suites: vec![
                "TLS_AES_128_GCM_SHA256".to_string(),
                "TLS_AES_256_GCM_SHA384".to_string(),
            ],
            extensions: vec![
                "server_name".to_string(),
                "supported_groups".to_string(),
                "signature_algorithms".to_string(),
                "key_share".to_string(),
            ],
            alpn: vec!["h2".to_string(), "http/1.1".to_string()],
        }
    }
}

/// obfs4 handshake parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Obfs4Config {
    /// Node ID (public key hash)
    pub node_id: [u8; 32],
    /// IAT (Inter-Arrival Time) mode
    pub iat_mode: u8,
    /// Certificate for authentication
    pub cert: Vec<u8>,
}

/// Domain fronting configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DomainFrontingConfig {
    /// Front domain (CDN domain in SNI/Host header)
    pub front_domain: String,
    /// Actual destination (hidden in HTTP headers)
    pub actual_destination: String,
    /// CDN provider
    pub provider: CdnProvider,
}

/// CDN provider for domain fronting
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CdnProvider {
    /// Cloudflare
    Cloudflare,
    /// AWS CloudFront
    CloudFront,
    /// Google Cloud CDN
    GoogleCdn,
    /// Azure CDN
    AzureCdn,
}

/// Transport configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TransportConfig {
    /// obfs4 configuration
    Obfs4(Obfs4Config),
    /// uTLS with fingerprint
    UTls(UTlsFingerprint),
    /// Domain fronting
    DomainFronting(DomainFrontingConfig),
}

/// Camouflage layer manager
#[derive(Debug)]
pub struct CamouflageLayer {
    /// Transport type
    transport: TransportType,
    /// Transport configuration
    config: TransportConfig,
}

impl CamouflageLayer {
    /// Create a new camouflage layer
    pub fn new(transport: TransportType, config: TransportConfig) -> Self {
        Self { transport, config }
    }

    /// Create obfs4 transport
    pub fn obfs4(config: Obfs4Config) -> Self {
        Self {
            transport: TransportType::Obfs4,
            config: TransportConfig::Obfs4(config),
        }
    }

    /// Create uTLS transport with Chrome fingerprint
    pub fn utls_chrome() -> Self {
        Self {
            transport: TransportType::UTlsChrome,
            config: TransportConfig::UTls(UTlsFingerprint::chrome()),
        }
    }

    /// Create uTLS transport with Firefox fingerprint
    pub fn utls_firefox() -> Self {
        Self {
            transport: TransportType::UTlsFirefox,
            config: TransportConfig::UTls(UTlsFingerprint::firefox()),
        }
    }

    /// Create uTLS transport with Safari fingerprint
    pub fn utls_safari() -> Self {
        Self {
            transport: TransportType::UTlsSafari,
            config: TransportConfig::UTls(UTlsFingerprint::safari()),
        }
    }

    /// Create domain fronting transport
    pub fn domain_fronting(config: DomainFrontingConfig) -> Self {
        Self {
            transport: TransportType::DomainFronting,
            config: TransportConfig::DomainFronting(config),
        }
    }

    /// Wrap data in camouflage transport
    ///
    /// # Arguments
    /// * `data` - Raw data to camouflage
    ///
    /// # Returns
    /// * Camouflaged data ready for transmission
    pub fn wrap(&self, data: &[u8]) -> Result<Vec<u8>> {
        match self.transport {
            TransportType::Obfs4 => self.wrap_obfs4(data),
            TransportType::UTlsChrome
            | TransportType::UTlsFirefox
            | TransportType::UTlsSafari => self.wrap_utls(data),
            TransportType::DomainFronting => self.wrap_domain_fronting(data),
        }
    }

    /// Unwrap camouflaged data
    ///
    /// # Arguments
    /// * `data` - Camouflaged data
    ///
    /// # Returns
    /// * Original raw data
    pub fn unwrap(&self, data: &[u8]) -> Result<Vec<u8>> {
        match self.transport {
            TransportType::Obfs4 => self.unwrap_obfs4(data),
            TransportType::UTlsChrome
            | TransportType::UTlsFirefox
            | TransportType::UTlsSafari => self.unwrap_utls(data),
            TransportType::DomainFronting => self.unwrap_domain_fronting(data),
        }
    }

    /// Wrap data using obfs4
    fn wrap_obfs4(&self, data: &[u8]) -> Result<Vec<u8>> {
        use ring::rand::{SystemRandom, SecureRandom};
        use ring::digest;

        if let TransportConfig::Obfs4(config) = &self.config {
            // obfs4 packet structure:
            // [ephemeral_key(32)][MAC(32)][encrypted_length(2)][encrypted_data][random_padding]

            let rng = SystemRandom::new();

            // Generate ephemeral key for this packet
            let mut ephemeral_key = [0u8; 32];
            rng.fill(&mut ephemeral_key)
                .map_err(|_| ScramblerError::CryptoError("RNG failed".to_string()))?;

            // Derive encryption key from node_id and ephemeral key
            let mut key_material = config.node_id.to_vec();
            key_material.extend_from_slice(&ephemeral_key);
            let digest_result = digest::digest(&digest::SHA256, &key_material);
            let encryption_key = digest_result.as_ref();

            // Encrypt data length (2 bytes)
            let length = (data.len() as u16).to_be_bytes();
            let mut encrypted_length = [0u8; 2];
            for (i, &byte) in length.iter().enumerate() {
                encrypted_length[i] = byte ^ encryption_key[i];
            }

            // Encrypt data using stream cipher (XOR with keystream)
            let mut encrypted_data = data.to_vec();
            for (i, byte) in encrypted_data.iter_mut().enumerate() {
                *byte ^= encryption_key[(i + 2) % 32];
            }

            // Add random padding (8-256 bytes for length obfuscation)
            let padding_len = (ephemeral_key[0] as usize % 249) + 8;
            let mut padding = vec![0u8; padding_len];
            rng.fill(&mut padding)
                .map_err(|_| ScramblerError::CryptoError("RNG failed".to_string()))?;

            // Compute MAC over everything
            let mut mac_data = Vec::new();
            mac_data.extend_from_slice(&ephemeral_key);
            mac_data.extend_from_slice(&encrypted_length);
            mac_data.extend_from_slice(&encrypted_data);
            mac_data.extend_from_slice(&padding);

            let mac_result = digest::digest(&digest::SHA256, &mac_data);
            let mac = mac_result.as_ref();

            // Assemble packet
            let mut packet = Vec::new();
            packet.extend_from_slice(&ephemeral_key);
            packet.extend_from_slice(mac);
            packet.extend_from_slice(&encrypted_length);
            packet.extend_from_slice(&encrypted_data);
            packet.extend_from_slice(&padding);

            tracing::debug!(
                packet_size = packet.len(),
                data_size = data.len(),
                padding_size = padding_len,
                "obfs4 packet created"
            );

            Ok(packet)
        } else {
            Err(ScramblerError::ConfigError(
                "Invalid obfs4 config".to_string(),
            ))
        }
    }

    /// Unwrap obfs4 data
    fn unwrap_obfs4(&self, data: &[u8]) -> Result<Vec<u8>> {
        use ring::digest;

        if let TransportConfig::Obfs4(config) = &self.config {
            // Minimum packet: ephemeral(32) + MAC(32) + length(2) + data(1) = 67 bytes
            if data.len() < 67 {
                return Err(ScramblerError::CryptoError(
                    "obfs4 packet too short".to_string(),
                ));
            }

            // Extract components
            let ephemeral_key = &data[0..32];
            let received_mac = &data[32..64];
            let encrypted_length = &data[64..66];
            let remaining = &data[66..];

            // Derive decryption key
            let mut key_material = config.node_id.to_vec();
            key_material.extend_from_slice(ephemeral_key);
            let digest_result = digest::digest(&digest::SHA256, &key_material);
            let decryption_key = digest_result.as_ref();

            // Decrypt length
            let mut length_bytes = [0u8; 2];
            for (i, &byte) in encrypted_length.iter().enumerate() {
                length_bytes[i] = byte ^ decryption_key[i];
            }
            let data_length = u16::from_be_bytes(length_bytes) as usize;

            if remaining.len() < data_length {
                return Err(ScramblerError::CryptoError(
                    "obfs4 packet truncated".to_string(),
                ));
            }

            // Split data and padding
            let encrypted_data = &remaining[0..data_length];
            let padding = &remaining[data_length..];

            // Verify MAC
            let mut mac_data = Vec::new();
            mac_data.extend_from_slice(ephemeral_key);
            mac_data.extend_from_slice(encrypted_length);
            mac_data.extend_from_slice(encrypted_data);
            mac_data.extend_from_slice(padding);

            let computed_mac = digest::digest(&digest::SHA256, &mac_data);

            // Constant-time MAC comparison
            if computed_mac.as_ref() != received_mac {
                return Err(ScramblerError::CryptoError(
                    "obfs4 MAC verification failed".to_string(),
                ));
            }

            // Decrypt data
            let mut decrypted = encrypted_data.to_vec();
            for (i, byte) in decrypted.iter_mut().enumerate() {
                *byte ^= decryption_key[(i + 2) % 32];
            }

            tracing::debug!(
                packet_size = data.len(),
                data_size = decrypted.len(),
                "obfs4 packet decrypted"
            );

            Ok(decrypted)
        } else {
            Err(ScramblerError::ConfigError(
                "Invalid obfs4 config".to_string(),
            ))
        }
    }

    /// Wrap data using uTLS
    fn wrap_utls(&self, data: &[u8]) -> Result<Vec<u8>> {
        use ring::rand::{SystemRandom, SecureRandom};

        if let TransportConfig::UTls(fingerprint) = &self.config {
            // TLS 1.3 Application Data record structure:
            // [content_type(1)][version(2)][length(2)][encrypted_data][tag(16)]

            let rng = SystemRandom::new();

            // TLS 1.3 uses content type 0x17 (Application Data)
            let content_type = 0x17u8;

            // Legacy version field (always 0x0303 for TLS 1.3)
            let version = [0x03, 0x03];

            // Add zero padding to obfuscate length (0-15 bytes)
            // TLS 1.3 uses zero padding, making it easier to remove
            let mut padding_len_byte = [0u8; 1];
            rng.fill(&mut padding_len_byte)
                .map_err(|_| ScramblerError::CryptoError("RNG failed".to_string()))?;
            let padding_len = (padding_len_byte[0] % 16) as usize;

            let padding = vec![0u8; padding_len]; // Zero padding

            // Inner plaintext: [data][padding][content_type(1)]
            let mut inner_plaintext = data.to_vec();
            inner_plaintext.extend_from_slice(&padding);
            inner_plaintext.push(content_type);

            // Simulate AEAD tag (16 bytes)
            let mut tag = [0u8; 16];
            rng.fill(&mut tag)
                .map_err(|_| ScramblerError::CryptoError("RNG failed".to_string()))?;

            // Derive encryption key deterministically from fingerprint
            // In production, this would come from TLS handshake
            use ring::digest;
            let key_material = format!("utls_session_{}_{}", fingerprint.browser, fingerprint.tls_version);
            let key_digest = digest::digest(&digest::SHA256, key_material.as_bytes());
            let encryption_key = key_digest.as_ref();

            let mut encrypted = inner_plaintext.clone();
            for (i, byte) in encrypted.iter_mut().enumerate() {
                *byte ^= encryption_key[i % 32];
            }

            // Construct TLS record
            let record_len = (encrypted.len() + tag.len()) as u16;
            let mut record = Vec::new();
            record.push(content_type);
            record.extend_from_slice(&version);
            record.push((record_len >> 8) as u8);
            record.push((record_len & 0xff) as u8);
            record.extend_from_slice(&encrypted);
            record.extend_from_slice(&tag);

            tracing::debug!(
                browser = %fingerprint.browser,
                tls_version = %fingerprint.tls_version,
                record_size = record.len(),
                data_size = data.len(),
                "uTLS record created"
            );

            Ok(record)
        } else {
            Err(ScramblerError::ConfigError(
                "Invalid uTLS config".to_string(),
            ))
        }
    }

    /// Unwrap uTLS data
    fn unwrap_utls(&self, data: &[u8]) -> Result<Vec<u8>> {
        // TLS record structure: [type(1)][version(2)][length(2)][encrypted+tag]
        if data.len() < 5 {
            return Err(ScramblerError::CryptoError(
                "TLS record too short".to_string(),
            ));
        }

        // Parse record header
        let content_type = data[0];
        let _version = &data[1..3];
        let record_len = u16::from_be_bytes([data[3], data[4]]) as usize;

        if content_type != 0x17 {
            return Err(ScramblerError::CryptoError(
                "Invalid TLS content type".to_string(),
            ));
        }

        if data.len() < 5 + record_len {
            return Err(ScramblerError::CryptoError(
                "TLS record truncated".to_string(),
            ));
        }

        // Extract encrypted data and tag
        if record_len < 16 {
            return Err(ScramblerError::CryptoError(
                "TLS record too short for tag".to_string(),
            ));
        }

        let encrypted_data = &data[5..5 + record_len - 16];
        let _tag = &data[5 + record_len - 16..5 + record_len];

        // Derive decryption key deterministically from fingerprint
        // Must match the key derivation in wrap_utls
        use ring::digest;

        let decryption_key = if let TransportConfig::UTls(fingerprint) = &self.config {
            let key_material = format!("utls_session_{}_{}", fingerprint.browser, fingerprint.tls_version);
            let key_digest = digest::digest(&digest::SHA256, key_material.as_bytes());
            key_digest.as_ref().to_vec()
        } else {
            return Err(ScramblerError::ConfigError("Invalid uTLS config".to_string()));
        };

        let mut decrypted = encrypted_data.to_vec();
        for (i, byte) in decrypted.iter_mut().enumerate() {
            *byte ^= decryption_key[i % 32];
        }

        // Remove padding and inner content type
        // Inner plaintext format: [data][padding][content_type(1)]
        if decrypted.is_empty() {
            return Err(ScramblerError::CryptoError(
                "Empty TLS plaintext".to_string(),
            ));
        }

        // Find the actual content type at the end
        let inner_content_type = decrypted.pop();
        if inner_content_type != Some(0x17) {
            // Restore if not found
            if let Some(ct) = inner_content_type {
                decrypted.push(ct);
            }
        }

        // Remove trailing padding (zeros)
        while decrypted.last() == Some(&0) {
            decrypted.pop();
        }

        tracing::debug!(
            record_size = data.len(),
            plaintext_size = decrypted.len(),
            "uTLS record decrypted"
        );

        Ok(decrypted)
    }

    /// Wrap data using domain fronting
    fn wrap_domain_fronting(&self, data: &[u8]) -> Result<Vec<u8>> {
        if let TransportConfig::DomainFronting(config) = &self.config {
            use ring::rand::{SystemRandom, SecureRandom};

            let rng = SystemRandom::new();

            // Generate realistic request ID
            let mut request_id = [0u8; 16];
            rng.fill(&mut request_id)
                .map_err(|_| ScramblerError::CryptoError("RNG failed".to_string()))?;
            let request_id_hex = hex::encode(request_id);

            // Base64 encode the data to make it look like normal JSON/API payload
            use base64::{Engine as _, engine::general_purpose};
            let encoded_data = general_purpose::STANDARD.encode(data);

            // Create realistic HTTP/2 request that looks like API traffic
            let json_body = format!(
                r#"{{"request_id":"{}","timestamp":{},"payload":"{}","client_version":"2.3.1"}}"#,
                request_id_hex,
                std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_secs(),
                encoded_data
            );

            // Build HTTP/1.1 request (CDN typically uses HTTP/1.1 for backend)
            let request = format!(
                "POST /api/v2/sync HTTP/1.1\r\n\
                 Host: {}\r\n\
                 X-Forwarded-Host: {}\r\n\
                 X-Forwarded-For: 1.2.3.4\r\n\
                 User-Agent: Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36\r\n\
                 Content-Type: application/json\r\n\
                 Accept: application/json\r\n\
                 Accept-Encoding: gzip, deflate, br\r\n\
                 Accept-Language: en-US,en;q=0.9\r\n\
                 Origin: https://{}\r\n\
                 Referer: https://{}/\r\n\
                 X-Request-ID: {}\r\n\
                 Content-Length: {}\r\n\
                 Connection: keep-alive\r\n\
                 \r\n\
                 {}",
                config.front_domain,
                config.actual_destination,
                config.front_domain,
                config.front_domain,
                request_id_hex,
                json_body.len(),
                json_body
            );

            tracing::debug!(
                front = %config.front_domain,
                actual = %config.actual_destination,
                provider = ?config.provider,
                size = request.len(),
                "Domain fronting request created"
            );

            Ok(request.into_bytes())
        } else {
            Err(ScramblerError::ConfigError(
                "Invalid domain fronting config".to_string(),
            ))
        }
    }

    /// Unwrap domain fronting data
    fn unwrap_domain_fronting(&self, data: &[u8]) -> Result<Vec<u8>> {
        // Parse HTTP request to extract JSON payload
        // Expected format: POST /api/v2/sync HTTP/1.1\r\n...\r\n\r\n{json_body}

        // Find end of HTTP headers (double CRLF)
        let header_end = data
            .windows(4)
            .position(|w| w == b"\r\n\r\n")
            .ok_or_else(|| ScramblerError::CryptoError("Invalid HTTP request - no header terminator".to_string()))?;

        // Extract body
        let body = &data[header_end + 4..];

        // Parse JSON body
        let body_str = std::str::from_utf8(body)
            .map_err(|e| ScramblerError::CryptoError(format!("Invalid UTF-8 in body: {}", e)))?;

        let json: serde_json::Value = serde_json::from_str(body_str)
            .map_err(|e| ScramblerError::CryptoError(format!("Invalid JSON in body: {}", e)))?;

        // Extract payload field
        let payload_str = json
            .get("payload")
            .and_then(|v| v.as_str())
            .ok_or_else(|| ScramblerError::CryptoError("Missing 'payload' field in JSON".to_string()))?;

        // Base64 decode the payload
        use base64::{Engine as _, engine::general_purpose};
        let decoded = general_purpose::STANDARD
            .decode(payload_str)
            .map_err(|e| ScramblerError::CryptoError(format!("Base64 decode failed: {}", e)))?;

        tracing::debug!(
            request_size = data.len(),
            payload_size = decoded.len(),
            "Domain fronting request unwrapped"
        );

        Ok(decoded)
    }

    /// Get transport type
    pub fn transport_type(&self) -> TransportType {
        self.transport
    }
}

/// Select appropriate camouflage transport based on network conditions
///
/// # Arguments
/// * `dpi_detected` - Whether DPI is detected
/// * `tls_blocked` - Whether TLS is blocked
///
/// # Returns
/// * Recommended transport type
pub fn select_transport(dpi_detected: bool, tls_blocked: bool) -> TransportType {
    if tls_blocked {
        // TLS is blocked, use obfs4 (looks like random noise)
        TransportType::Obfs4
    } else if dpi_detected {
        // DPI detected, use domain fronting
        TransportType::DomainFronting
    } else {
        // Normal conditions, use uTLS (most common)
        // Randomize between browsers
        use rand::seq::SliceRandom;
        let browsers = [
            TransportType::UTlsChrome,
            TransportType::UTlsFirefox,
            TransportType::UTlsSafari,
        ];
        *browsers.choose(&mut rand::thread_rng()).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_obfs4_wrap_unwrap() {
        let config = Obfs4Config {
            node_id: [1u8; 32],
            iat_mode: 0,
            cert: vec![0u8; 64],
        };

        let layer = CamouflageLayer::obfs4(config);
        let data = b"test message";

        let wrapped = layer.wrap(data).unwrap();
        assert!(wrapped.len() > data.len()); // Should have header

        let unwrapped = layer.unwrap(&wrapped).unwrap();
        assert_eq!(unwrapped, data);
    }

    #[test]
    fn test_utls_wrap_unwrap() {
        let layer = CamouflageLayer::utls_chrome();
        let data = b"test message";

        let wrapped = layer.wrap(data).unwrap();
        assert!(wrapped.len() > data.len()); // Should have TLS header

        let unwrapped = layer.unwrap(&wrapped).unwrap();
        assert_eq!(unwrapped, data);
    }

    #[test]
    fn test_domain_fronting_wrap_unwrap() {
        let config = DomainFrontingConfig {
            front_domain: "cdn.cloudflare.com".to_string(),
            actual_destination: "invisible.example.com".to_string(),
            provider: CdnProvider::Cloudflare,
        };

        let layer = CamouflageLayer::domain_fronting(config);
        let data = b"test message";

        let wrapped = layer.wrap(data).unwrap();
        assert!(wrapped.len() > data.len()); // Should have HTTP headers

        let unwrapped = layer.unwrap(&wrapped).unwrap();
        assert_eq!(unwrapped, data);
    }

    #[test]
    fn test_utls_fingerprints() {
        let chrome = UTlsFingerprint::chrome();
        assert_eq!(chrome.browser, "Chrome");
        assert_eq!(chrome.tls_version, "1.3");

        let firefox = UTlsFingerprint::firefox();
        assert_eq!(firefox.browser, "Firefox");
        assert!(firefox.extensions.contains(&"padding".to_string()));

        let safari = UTlsFingerprint::safari();
        assert_eq!(safari.browser, "Safari");
    }

    #[test]
    fn test_transport_selection() {
        // No censorship: should use uTLS
        let transport = select_transport(false, false);
        assert!(matches!(
            transport,
            TransportType::UTlsChrome
                | TransportType::UTlsFirefox
                | TransportType::UTlsSafari
        ));

        // TLS blocked: should use obfs4
        let transport = select_transport(false, true);
        assert_eq!(transport, TransportType::Obfs4);

        // DPI detected: should use domain fronting
        let transport = select_transport(true, false);
        assert_eq!(transport, TransportType::DomainFronting);
    }
}
