# Invisible - Production Deployment Checklist

This document provides a comprehensive checklist for deploying Invisible to production across all platforms.

## Pre-Deployment Verification

### Code Quality
- [ ] All Rust tests passing: `cargo test --workspace` (75/75 tests)
- [ ] All Flutter tests passing: `flutter test`
- [ ] Integration tests passing: `flutter test integration_test/`
- [ ] No compiler warnings in release builds
- [ ] Code review completed for all changes
- [ ] Security audit completed (external if possible)

### FFI Integration
- [ ] All FFI bindings generated: `cd invisible_app/rust && cargo build`
- [ ] macOS library builds: `./build.sh` completes successfully
- [ ] iOS library builds (if targeting iOS)
- [ ] Android library builds (if targeting Android)
- [ ] FFI functions tested with integration tests

### Security
- [ ] Argon2 parameters verified (256 MB, 4 iterations, 4 parallelism)
- [ ] TOTP 2FA working with real authenticator app
- [ ] Panic wipe triggers correctly after 5 failed attempts
- [ ] Rate limiting prevents brute force attacks
- [ ] Secure storage using platform keychain
- [ ] No sensitive data in logs or crash reports
- [ ] Debug mode disabled in release builds
- [ ] All TODOs for security issues resolved

### Performance
- [ ] App launch time < 2 seconds
- [ ] PIN verification time acceptable (~100-500ms for Argon2)
- [ ] Wallet balance refresh < 5 seconds
- [ ] Message send/receive latency acceptable
- [ ] Memory usage reasonable (< 200MB idle)
- [ ] Battery drain acceptable

---

## Backend Infrastructure

### Relay Nodes
- [ ] Deploy 5+ relay nodes globally distributed
- [ ] Configure Scrambler mixnet (5-layer minimum)
- [ ] Set up dead drop servers for async message delivery
- [ ] Configure cover traffic generation
- [ ] Enable Tor fallback routing

### VPN Endpoints
- [ ] Deploy Ghost VPN endpoints (WireGuard)
- [ ] Distribute across multiple jurisdictions
- [ ] Avoid Five Eyes countries where possible
- [ ] Configure random endpoint selection

### Blockchain Nodes
- [ ] Monero (XMR) full node or trusted RPC
- [ ] Zcash (ZEC) full node or trusted RPC
- [ ] Bitcoin (BTC) full node or Electrum server
- [ ] Ethereum (ETH) full node or Infura/Alchemy

### Monitoring
- [ ] Set up uptime monitoring (relay nodes, VPN endpoints)
- [ ] Configure alerting for node failures
- [ ] Monitor network traffic patterns
- [ ] Track error rates and crash reports
- [ ] Privacy-preserving analytics (no user tracking)

---

## Mobile App Builds

### iOS

**Prerequisites:**
- [ ] Apple Developer account ($99/year)
- [ ] Code signing certificate
- [ ] Provisioning profiles
- [ ] App Store Connect configured

**Build:**
```bash
cd invisible_app
flutter build ios --release
```

**Distribution:**
- [ ] Upload to TestFlight for beta testing
- [ ] Test on multiple iOS versions (15.0+)
- [ ] Submit for App Store review
- [ ] Prepare App Store listing (screenshots, description, privacy policy)

**App Store Compliance:**
- [ ] Privacy policy URL
- [ ] Export compliance documentation (encryption)
- [ ] Age rating: 17+ (unrestricted web access)

---

### Android

**Prerequisites:**
- [ ] Google Play Developer account ($25 one-time)
- [ ] Keystore for code signing
- [ ] Google Play Console configured

**Build:**
```bash
cd invisible_app
flutter build apk --release --split-per-abi  # or
flutter build appbundle --release  # for Play Store
```

**Distribution:**
- [ ] Upload APK/AAB to Google Play Console
- [ ] Test on multiple Android versions (8.0+)
- [ ] Submit for review
- [ ] Prepare Play Store listing

**Play Store Compliance:**
- [ ] Privacy policy URL
- [ ] Data safety section completed
- [ ] Target API level 33+ (Android 13)
- [ ] Content rating questionnaire

---

### macOS

**Prerequisites:**
- [ ] Apple Developer account
- [ ] Code signing certificate
- [ ] Notarization setup

**Build:**
```bash
cd invisible_app
flutter build macos --release
```

**Distribution:**
- [ ] Code sign app bundle
- [ ] Notarize with Apple
- [ ] Create DMG installer
- [ ] Distribute via website or Mac App Store

---

### Linux

**Build:**
```bash
cd invisible_app
flutter build linux --release
```

**Distribution:**
- [ ] Create .deb package (Debian/Ubuntu)
- [ ] Create .rpm package (Fedora/RedHat)
- [ ] Create AppImage (universal)
- [ ] Distribute via website

---

## Post-Deployment

### Monitoring
- [ ] Monitor crash reports (Sentry, Firebase Crashlytics)
- [ ] Track error rates in production
- [ ] Monitor performance metrics
- [ ] Check relay node health

### User Support
- [ ] Set up support channels (email, forum, etc.)
- [ ] Create user documentation
- [ ] Prepare FAQ
- [ ] Establish response protocols for security issues

### Updates
- [ ] Plan regular security updates
- [ ] Monitor dependency vulnerabilities: `cargo audit`
- [ ] Keep Flutter and dependencies up to date
- [ ] Test updates thoroughly before release

### Security
- [ ] Establish responsible disclosure policy
- [ ] Monitor for security vulnerabilities
- [ ] Plan penetration testing schedule
- [ ] Regular security audits (annual minimum)

---

## Rollback Plan

If critical issues discovered:
- [ ] Document rollback procedure
- [ ] Maintain previous stable version
- [ ] Have hotfix process ready
- [ ] Communication plan for users

---

## Success Criteria

**Before Launch:**
- All checklists above completed
- No critical bugs or security issues
- Performance meets targets
- Backend infrastructure stable

**After Launch:**
- < 1% crash rate
- < 5% error rate
- Positive user feedback
- No security incidents

---

## Estimated Timeline

- **Pre-deployment checks:** 2-3 days
- **Backend setup:** 3-5 days
- **iOS/Android builds:** 1 day
- **App store review:** 1-2 weeks
- **Total:** ~3-4 weeks from code freeze to production

---

## Next Steps After Deployment

1. **Gather user feedback** - Monitor reviews and support requests
2. **Performance optimization** - Address any bottlenecks
3. **Feature development** - Implement remaining TODO items
4. **Security audits** - Schedule external security review
5. **Marketing** - User acquisition and community building

---

**🚀 Ready to deploy Invisible with 100% Flutter-Rust FFI integration!**
