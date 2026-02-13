# Invisible — Comprehensive Project Plan

**Version:** 2.0.0
**Created:** 2026-02-12
**Updated:** 2026-02-13
**Status:** Active Development Planning

---

## Executive Summary

**Mission:** Build the world's most private messenger with zero-compromise privacy and maximum usability. Every feature—from messaging to payments—protected by 7-layer network obfuscation, with user experience so intuitive that privacy becomes invisible.

**Scope:**
- **Timeline:** 88 weeks across 7 phases
- **Team:** 25-40 person comprehensive team with specialized roles
- **Deliverable:** Production-ready Invisible messenger with mobile apps, relay infrastructure, and community node network

**Approach:**
- **Design-Driven Development:** UX/UI team leads feature design 6 weeks ahead of engineering. No code until designs are validated with users.
- **Continuous Security Validation:** Security testing in every 2-week sprint, not just at phase boundaries. Red team continuously attacks the system.
- **Sprint-Based Execution:** 2-week sprint cycles with clear deliverables, team assignments, and success criteria.
- **Deployment Automation:** Production-ready infrastructure from day one with CI/CD pipelines and reproducible builds.

**Key Principles:**
- **Privacy Parity:** Messages and money receive equal protection—full 7-layer Scrambler for all operations
- **UX Obsession:** If users can't figure it out in 30 seconds, we failed
- **Security First:** Every decision starts with "What does an adversary learn?"
- **Defense in Depth:** Multiple independent security layers—an adversary must defeat ALL simultaneously
- **Open Everything:** All code, protocols, and crypto implementations are open source with reproducible builds

---

## Table of Contents

**Planning & Process**
1. [Executive Summary](#executive-summary)
2. [Team Structure & Roles](#team-structure--roles-25-40-people)
3. [Development Workflow & Sprint Process](#development-workflow--sprint-process)
4. [UX/UI Design-Driven Development](#uxui-design-driven-development)
5. [Security & QA Continuous Validation](#security--qa-continuous-validation)
6. [Deployment & Infrastructure Strategy](#deployment--infrastructure-strategy)
7. [Communication & Collaboration](#communication--collaboration)
8. [Technology Stack & Tools](#technology-stack--tools)

**Phase Execution**
9. [Phase 0: Foundation (Weeks 1-8)](#phase-0-foundation-weeks-1-8)
10. [Phase 1: Core Messaging (Weeks 9-16)](#phase-1-core-messaging-weeks-9-16)
11. [Phase 2: The Scrambler (Weeks 17-28)](#phase-2-the-scrambler-weeks-17-28)
12. [Phase 3: Ghost VPN + Hardening (Weeks 29-36)](#phase-3-ghost-vpn--hardening-weeks-29-36)
13. [Phase 4: Groups + Media + Calls (Weeks 37-48)](#phase-4-groups--media--calls-weeks-37-48)
14. [Phase 5: Shadow Wallet + Crypto (Weeks 49-64)](#phase-5-shadow-wallet--crypto-weeks-49-64)
15. [Phase 6: Mesh + Community Nodes (Weeks 65-76)](#phase-6-mesh--community-nodes-weeks-65-76)
16. [Phase 7: Audit + Launch (Weeks 77-88)](#phase-7-audit--launch-weeks-77-88)

**Supporting Information**
17. [Dependencies](#dependencies)
18. [Success Metrics & Monitoring](#success-metrics--monitoring)
19. [Risk Management & Mitigation](#risk-management--mitigation)
20. [Shadow Wallet Security Hardening](#shadow-wallet-security-hardening)
21. [Network Privacy Mode](#network-privacy-mode-cellular-metadata-protection)
22. [Admin Dashboard](#admin-dashboard-privacy-control-center)

---

## Team Structure & Roles (25-40 People)

**Team philosophy:** Specialized excellence over generalist coverage. Each domain (crypto, mobile, backend, wallet, design, security, DevOps) has dedicated experts who own their area end-to-end.

### Core Leadership (4 people)

**Product Lead** (1)
- Responsibilities: Vision, roadmap, stakeholder alignment, prioritization
- Key Skills: Product strategy, user empathy, technical fluency
- Reports To: CEO/Founders

**Technical Architect** (1)
- Responsibilities: System design, technical decisions, security architecture, cross-team technical alignment
- Key Skills: Distributed systems, cryptography, threat modeling
- Reports To: CEO/Founders

**UX/UI Design Lead** (1)
- Responsibilities: Design vision, user research, design system ownership, design team management
- Key Skills: User research, interaction design, accessibility, design systems
- Reports To: Product Lead

**Engineering Lead** (1)
- Responsibilities: Team coordination, code quality, technical mentorship, process optimization
- Key Skills: People management, architecture review, delivery optimization
- Reports To: Technical Architect

---

### Engineering Teams (18-22 people)

#### Cryptography & Core Team (4-5)

**Team Mission:** Build and maintain the cryptographic foundation—Signal Protocol, post-quantum crypto, key management, and core security primitives.

**Roles:**
- **Rust Engineers** (2): Signal Protocol Double Ratchet, X3DH, PQXDH, Ed25519 signatures
- **Security Engineer** (1): Crypto validation, threat modeling, constant-time operations review
- **Performance Engineer** (1): Benchmark crypto operations, optimize hot paths, memory profiling
- **Test Engineer** (1): Property testing, fuzzing, known-answer tests (NIST vectors)

**Key Responsibilities:**
- Implement and maintain libsignal-protocol integration
- Post-quantum key exchange (ML-KEM + X25519 hybrid)
- Local storage encryption (SQLCipher + Argon2id KDF)
- Daily key rotation system
- All crypto code must achieve 100% branch coverage with property tests

---

#### Mobile Team (5-6)

**Team Mission:** Build beautiful, intuitive Flutter mobile apps for iOS and Android that make privacy invisible to users.

**Roles:**
- **Flutter Engineers** (3): Cross-platform mobile development, state management, UI implementation
- **iOS Platform Specialist** (1): Native iOS integration (Secure Enclave, biometrics, permissions)
- **Android Platform Specialist** (1): Native Android integration (StrongBox, biometrics, permissions)
- **Mobile QA Engineer** (1): Device testing, platform-specific bugs, accessibility testing

**Key Responsibilities:**
- Flutter mobile app development (iOS/Android)
- Native platform integration (biometrics, hardware security modules)
- Implement designs from UX team with pixel-perfect accuracy
- Performance optimization (battery usage, startup time, memory)
- Anti-forensic features (no thumbnails, blank task switcher, secure keyboard)

---

#### Backend/Infrastructure Team (5-6)

**Team Mission:** Build the relay, mixnet, and VPN infrastructure that provides unbreakable network privacy.

**Roles:**
- **Rust Engineers** (3): Relay nodes, mixnet implementation (Sphinx packets, Loopix), VPN integration
- **Network Engineer** (1): Protocol design, performance optimization, bandwidth efficiency
- **Database Engineer** (1): SQLCipher schema design, data models, migration strategy
- **Infrastructure Automation Engineer** (1): Terraform, Ansible, Docker, Kubernetes deployment

**Key Responsibilities:**
- Relay node software (RAM-only, stateless)
- 7-layer Scrambler implementation (Shamir fragmentation, 5-layer mixnet, cover traffic, etc.)
- Ghost VPN integration (WireGuard, endpoint rotation)
- Dead drop architecture (anonymous message mailboxes)
- Node deployment automation (one-click community setup)

---

#### Wallet/DeFi Team (4-5)

**Team Mission:** Build Shadow Wallet—a non-custodial, privacy-first crypto wallet with fortress-grade security.

**Roles:**
- **Blockchain Engineers** (2): Monero, Zcash, Bitcoin, Ethereum integration
- **Smart Contract Security Engineer** (1): DeFi proxy security, contract verification, approval limits
- **DeFi Integration Engineer** (1): WalletConnect v2, RPC proxy, swap protocols
- **Wallet QA Engineer** (1): Security testing, transaction validation, recovery testing

**Key Responsibilities:**
- Privacy coin integration (Monero, Zcash, Bitcoin with CoinJoin/PayJoin)
- Ethereum + stablecoin support (ZK-rollup privacy layers)
- Phantom Swap (atomic cross-chain swaps via HTLC)
- Anonymous DeFi proxy (WalletConnect through Scrambler)
- Hardware security module integration (Secure Enclave, StrongBox)
- Transaction verification layer (anti-phishing, address validation)

---

### Design & UX Team (5-6 people)

**Team Mission:** Lead feature design 6 weeks ahead of engineering. Ensure every feature is so intuitive that users understand it in 30 seconds.

**Roles:**
- **UX/UI Design Lead** (1): Design vision, design system ownership (also in Core Leadership)
- **Product Designers** (2): User flows, wireframes, high-fidelity prototypes, user testing
- **Visual Designer** (1): Design system components, UI polish, iconography, animations
- **UX Researcher** (1): User interviews, usability testing, validation studies, competitive analysis
- **Accessibility Specialist** (1): WCAG 2.1 AA compliance, screen reader optimization, inclusive design

**Key Responsibilities:**
- Conduct user research (interviews with journalists, activists, privacy-conscious users)
- Create and validate designs 6 weeks before engineering starts
- Build and maintain comprehensive design system (Figma)
- User testing (8-10 sessions per feature)
- Design QA on every engineering implementation
- Accessibility audits at every phase boundary

**The UX Obsession:**
- If users can't figure out a feature in 30 seconds, it failed
- Privacy features must be invisible (automatic) or obvious (one tap)
- Error messages must be helpful, never technical
- Every screen answers: "Where am I? What can I do? What happens next?"

---

### Security & QA (4-5 people)

**Team Mission:** Continuously attack the system to find vulnerabilities before adversaries do. Ensure every feature meets fortress-grade security standards.

**Roles:**
- **Security Lead** (1): Red team operations, penetration testing, threat modeling leadership
- **Security Engineers** (2): Code review (crypto/wallet/network), security audits, threat modeling
- **QA Lead** (1): Test strategy, automation framework design, release validation
- **QA Engineer** (1): Integration testing, end-to-end testing, regression testing

**Key Responsibilities:**
- Security code review (2 security engineers for all crypto/wallet code)
- Threat modeling sessions (every sprint)
- Penetration testing (continuously attack new features)
- Fuzzing infrastructure (24/7 fuzzing of parsers, protocol handlers)
- Test coverage enforcement (85% overall, 100% for crypto/wallet)
- External audit coordination (NCC Group, Trail of Bits, Cure53)

**Red Team Philosophy:**
- 2 security engineers act as adversaries
- Simulate nation-state level attacks
- Test social engineering vectors
- Attempt deanonymization
- Try to extract keys from seized devices
- Report findings weekly

---

### DevOps & Infrastructure (3-4 people)

**Team Mission:** Make deployment effortless—one command to set up local dev, automatic deployment to staging, reproducible builds for production.

**Roles:**
- **DevOps Lead** (1): CI/CD pipeline, deployment automation, infrastructure as code
- **Infrastructure Engineer** (1): Cloud deployment (AWS, GCP, Azure), relay node hosting, monitoring
- **Release Engineer** (1): App store publishing (iOS App Store, Google Play, F-Droid), reproducible builds
- **SRE** (1) [Optional, can scale later]: Production monitoring, incident response, uptime optimization

**Key Responsibilities:**
- CI/CD pipeline (GitHub Actions): lint, test, security scan, build, deploy
- Infrastructure as code (Terraform + Ansible)
- One-click relay node deployment for community operators
- Reproducible builds (verify binaries match source)
- Monitoring and observability (Prometheus + Grafana, Sentry)
- App store release management

**Deployment Promises:**
- Local dev setup: <10 minutes (`./scripts/dev-setup.sh`)
- Community node setup: <15 minutes (`docker-compose up -d`)
- CI/CD: PR to staging in <30 minutes
- Production release: staging to app stores in <2 hours (with approvals)

---

### Supporting Roles (2-3 people)

**Technical Writer** (1)
- User documentation, API documentation, developer guides
- Onboarding materials for new team members
- Release notes and changelog maintenance

**Community Manager** (1) [Optional, if going public]
- Coordinate community relay node operators
- Open source community management
- Bug bounty program coordination

**Project Manager** (1) [Optional, can be handled by team leads]
- Sprint planning coordination
- Cross-team dependency tracking
- Timeline management and reporting

---

### Team Scaling Strategy

**Phase 0-1 (Weeks 1-16): Start Small - 25-30 people**
- Core Leadership: 4
- Crypto Team: 5 (full team)
- Mobile Team: 4 (start with 3 Flutter + 1 iOS specialist)
- Backend Team: 4 (3 Rust engineers + 1 network engineer)
- Design Team: 4 (design lead + 2 product designers + 1 UX researcher)
- Security & QA: 4 (security lead + 1 security eng + QA lead + 1 QA eng)
- DevOps: 3 (DevOps lead + infrastructure engineer + release engineer)
- Technical Writer: 1
- **Total: 29 people**

**Phase 2-3 (Weeks 17-36): Scale Up - 35-40 people**
- Add: Android platform specialist, 2nd security engineer, database engineer, wallet team (4), visual designer
- Full operational capacity with all specialized roles
- **Total: 38 people**

**Phase 4-6 (Weeks 37-76): Maintain - 40-45 people**
- Add: Accessibility specialist, SRE, wallet QA engineer
- Parallel workstreams on groups, wallet, mesh
- **Total: 41 people**

**Phase 7 (Weeks 77-88): Scale Down - 15-20 people**
- Audit support, polish, launch preparation
- Reduced team focused on bug fixes and audit remediation
- **Total: 18 people**

---

## Development Workflow & Sprint Process

**Core Principle:** 2-week sprint cycles with design-led development, continuous security validation, and rigorous quality gates.

### Sprint Structure

**Week 1:**
- **Monday**: Sprint Planning (4 hours)
  - Review design deliverables from previous sprint
  - Break down validated designs into engineering tasks
  - Security threat modeling for new features
  - Capacity planning and task assignment

- **Tuesday-Thursday**: Development
  - Daily standups (15 min per team)
  - Pair programming for crypto/security code
  - Continuous code review (no PR older than 24 hours)

- **Friday**:
  - Demo to design team (validate implementation matches design)
  - Security code review sessions
  - Dependency updates and security audits

**Week 2:**
- **Monday-Wednesday**: Development + Testing
  - Feature completion
  - Integration testing
  - Security testing (penetration testing for new attack surface)

- **Thursday**:
  - Sprint demo to full team (show working features)
  - Usability testing sessions with design team
  - Performance benchmarking

- **Friday**:
  - Retrospective (what worked, what didn't)
  - Code freeze for sprint
  - Deploy to internal staging environment
  - Next sprint prep (design team presents validated designs)

### Design-Led Development Process

**Before Development Starts (Design Sprint -1):**
1. **User Research** - UX team conducts user interviews, analyzes pain points
2. **Design Exploration** - 2-3 design directions explored
3. **Prototyping** - High-fidelity interactive prototypes built
4. **User Validation** - 5-8 user testing sessions
5. **Design Handoff** - Validated designs, specs, and Figma files delivered to engineering

**Engineering can only start when:**
- ✅ Design has been validated with real users
- ✅ Design specs are complete with measurements, interactions, edge cases
- ✅ Security implications have been reviewed
- ✅ Accessibility requirements are defined

**During Development:**
- Daily design/eng sync (15 min) - catch issues early
- Design QA every 2 days - validate implementation matches intent
- Final design approval before sprint demo

### Code Quality Gates

**Every PR must have:**
- ✅ Unit tests (80%+ coverage for new code)
- ✅ Integration tests (if touching API boundaries)
- ✅ Security review (2 security engineers for crypto/wallet code)
- ✅ Design approval (for UI changes)
- ✅ Performance benchmarks (no regressions)
- ✅ Documentation updates

**Automated CI/CD Pipeline:**
- Linting (rustfmt, clippy, Flutter analyzer)
- Unit test suite (must pass 100%)
- Security scanning (cargo-audit, dependency checks)
- Fuzzing (continuous fuzzing for parsers/crypto)
- Performance regression tests
- Build verification (reproducible builds)

### Definition of Done

**A feature is only "done" when:**
1. ✅ Code reviewed and merged
2. ✅ All tests passing (unit, integration, security)
3. ✅ Design QA approved
4. ✅ Security testing completed
5. ✅ Documentation written
6. ✅ Deployed to staging
7. ✅ Validated in sprint demo
8. ✅ No critical bugs filed

---

## UX/UI Design-Driven Development

**Core Principle: No code until design is validated with users**

### Design-First Philosophy

**The UX/UI team is not a service team - they lead feature development:**
- Design discovers and validates user needs BEFORE engineering builds
- Every feature starts with user research, not technical requirements
- "Can we build it?" comes after "Should we build it?" and "Will users understand it?"
- Engineers implement validated solutions, not experimental ideas

### The Design Process (4-6 weeks ahead of engineering)

**Phase 1: Discover (Week -6 to -5)**
- **User Research**: Interviews with target users (journalists, activists, privacy-conscious users)
- **Competitive Analysis**: What do Signal, Session, SimpleX do? Where do they fail?
- **Pain Point Mapping**: What frustrates users about existing privacy tools?
- **Opportunity Identification**: Where can Invisible be dramatically better?
- **Deliverable**: Research report with key insights and opportunity areas

**Phase 2: Define (Week -5 to -4)**
- **User Personas**: Detailed profiles of primary user types
- **User Journeys**: Map complete flows (onboarding, first message, adding contact, etc.)
- **Feature Requirements**: What must this feature do? What's nice-to-have?
- **Success Metrics**: How do we know if this feature succeeds?
- **Deliverable**: Feature brief with user stories, success criteria, constraints

**Phase 3: Design (Week -4 to -2)**
- **Exploration**: Design team creates 2-3 different approaches
- **Internal Critique**: Design team reviews, debates trade-offs
- **Prototyping**: Build high-fidelity interactive prototypes (Figma)
- **Accessibility Check**: Ensure designs meet WCAG 2.1 AA standards
- **Security Review**: Check with security team - does design leak metadata?
- **Deliverable**: 2-3 high-fidelity design directions

**Phase 4: Validate (Week -2 to -1)**
- **User Testing**: 8-10 sessions with target users
  - Task-based testing (can they complete core flows?)
  - Think-aloud protocol (what confuses them?)
  - Edge case exploration (what breaks the mental model?)
- **Iterate**: Refine design based on user feedback
- **Final Testing**: Validate refined design works
- **Security Validation**: Ensure UX doesn't compromise privacy
- **Deliverable**: Validated design with user testing insights

**Phase 5: Handoff (Week -1)**
- **Design Specs**: Measurements, spacing, typography, colors, interactions
- **Component Documentation**: How each UI element behaves
- **Edge Cases**: Loading states, errors, empty states, offline mode
- **Animation Specs**: Timing, easing, micro-interactions
- **Accessibility Requirements**: Screen reader behavior, keyboard navigation
- **Engineering Sync**: Walk through design, answer questions
- **Deliverable**: Complete design specs ready for implementation

### Design System (Built in Phase 0-1)

**Before Phase 2 begins, we build a comprehensive design system:**

**Foundation**
- Color palette (optimized for dark mode, high contrast)
- Typography scale (accessible, readable)
- Spacing system (8px grid)
- Iconography (custom icon set for privacy features)

**Components**
- Buttons (primary, secondary, destructive, disabled states)
- Input fields (text, secure, validation states)
- Cards, lists, navigation
- Modals, sheets, toasts
- Progress indicators (especially for Scrambler layers)

**Patterns**
- Onboarding flows
- Contact management
- Message bubbles
- Settings screens
- Error handling
- Loading states

**Accessibility**
- WCAG 2.1 AA compliance minimum
- Screen reader optimization
- High contrast mode
- Reduced motion support
- Keyboard navigation

### Continuous UX Validation

**Every Sprint:**
- **Design QA**: Design team reviews every UI implementation
- **Usability Testing**: 3-5 users test new features in staging
- **Analytics Review**: How are users actually using features? (privacy-preserving analytics only)
- **Bug Triage**: Design team prioritizes UX bugs vs. feature work

**Every Phase:**
- **Comprehensive Usability Study**: 15-20 users test complete feature set
- **Accessibility Audit**: External audit for WCAG compliance
- **Cognitive Walkthrough**: Can first-time users complete critical tasks?

**The UX Obsession:**
- If users can't figure out a feature in 30 seconds, we failed
- Privacy features must be invisible (automatic) or obvious (one tap)
- Error messages must be helpful, not technical
- Every screen must answer: "Where am I? What can I do? What happens next?"

---

## Security & QA Continuous Validation

**Core Principle: Security testing in every sprint, not just at the end**

### Continuous Security Validation

**Every Sprint (2 weeks):**

**Security Code Review (ongoing)**
- **All crypto code**: 2 security engineers must review + approve
- **Wallet code**: 2 security engineers + 1 blockchain specialist
- **Network code**: 1 security engineer + 1 network specialist
- **No exceptions**: Security-critical code cannot merge without security approval

**Threat Modeling Sessions (Monday of Week 1)**
- Review new features being built this sprint
- Identify attack vectors and threat scenarios
- Update threat model documentation
- Define security test cases for the sprint

**Security Testing (Week 2)**
- **Penetration Testing**: Red team attacks new features
- **Fuzzing**: Continuous fuzzing of parsers, protocol handlers
- **Dependency Scanning**: Check for vulnerable dependencies
- **Code Analysis**: Static analysis with clippy, cargo-audit
- **Memory Safety**: Valgrind, AddressSanitizer for unsafe code

**Security Checklist (before sprint demo)**
- ✅ No secrets in logs or error messages
- ✅ All crypto uses constant-time operations
- ✅ Sensitive data properly zeroized
- ✅ No timing side-channels in authentication
- ✅ Input validation on all external data
- ✅ Proper error handling (fail secure, not open)

### Phase-Level Security Validation

**At the end of each phase:**

**External Security Audit (mini-audit)**
- Engage external security firm (NCC Group, Trail of Bits, Cure53)
- Focus on code completed in that phase
- 1-2 week engagement
- Remediate all findings before next phase

**Security Regression Testing**
- Re-run all previous security tests
- Ensure new features didn't break existing security
- Validate defense-in-depth still intact

**Formal Verification (for critical crypto)**
- Phase 0: Verify Double Ratchet implementation
- Phase 2: Verify Sphinx packet format
- Phase 5: Verify wallet key derivation

### Continuous Quality Assurance

**Test Coverage Requirements:**
- **Crypto code**: 100% branch coverage (property testing required)
- **Wallet code**: 100% branch coverage + extensive integration tests
- **Network code**: 90% coverage + protocol conformance tests
- **UI code**: 80% coverage + visual regression tests
- **Overall**: Minimum 85% coverage across codebase

**Testing Strategy:**

**Unit Tests (every PR)**
- Fast tests (<5 sec total runtime)
- Test individual functions/methods
- Mock external dependencies
- Known-answer tests for crypto (NIST test vectors)

**Integration Tests (daily)**
- Test component interactions
- Real database, real crypto, real network
- End-to-end message flow tests
- Wallet transaction tests (on testnets)

**Property Testing (crypto & wallet)**
- Use proptest/quickcheck
- Verify invariants hold (e.g., encrypt→decrypt = identity)
- Find edge cases humans miss
- Run 10,000+ randomized test cases

**Fuzzing (continuous)**
- AFL/cargo-fuzz on parsers
- Fuzz packet formats (Sphinx, protocol buffers)
- Fuzz API inputs
- Run 24/7 on dedicated fuzzing infrastructure
- Crash triage within 24 hours

**Performance Testing (every sprint)**
- Benchmark crypto operations (must be <10ms for common ops)
- Load testing (relay nodes must handle 10k messages/sec)
- Memory profiling (no leaks, no unbounded growth)
- Battery impact testing (mobile app must not drain battery)

**Security Testing (every sprint)**
- OWASP Top 10 checks
- SQL injection testing (even with SQLCipher)
- XSS testing (if any web components)
- Authentication bypass attempts
- Privilege escalation tests
- Side-channel analysis (timing attacks)

### QA Release Process

**Before any deployment (staging or production):**

1. **Automated Test Suite** (CI/CD)
   - All unit tests pass
   - All integration tests pass
   - Security scans clean
   - No performance regressions

2. **Manual QA Testing**
   - Smoke tests on real devices (iOS/Android)
   - Critical user flows (onboarding, messaging, payments)
   - Edge case testing (poor network, low battery, etc.)
   - Accessibility testing (screen reader, high contrast)

3. **Security Validation**
   - Security checklist complete
   - Penetration test findings remediated
   - Dependency audit clean
   - No known critical vulnerabilities

4. **Design QA**
   - Visual regression tests pass
   - Design team approves implementation
   - Usability testing complete

5. **Sign-Off**
   - Engineering Lead ✓
   - Security Lead ✓
   - QA Lead ✓
   - Design Lead ✓

**Red Team (Dedicated Attackers)**
- 2 security engineers act as adversaries
- Continuously try to break the system
- Simulate nation-state level attacks
- Test social engineering vectors
- Attempt to deanonymize users
- Try to extract keys from seized devices
- Report findings weekly

**Bug Bounty Program (Phase 6+)**
- Launch public bug bounty
- Rewards: $500-$50,000 depending on severity
- Scope: All client code, relay code, protocol
- Out of scope: Social engineering, DoS

---

## Deployment & Infrastructure Strategy

**Core Principle: Deployment automation from day one, production-ready infrastructure**

### Deployment Environments

**1. Local Development**
- **Docker Compose** setup for complete local environment
- Local relay node, local VPN endpoint, local database
- Hot reload for rapid development
- Mock services for blockchain/external APIs
- One command setup: `./scripts/dev-setup.sh`

**2. CI/CD (Continuous Integration)**
- **GitHub Actions** for all automation
- Runs on every PR: lints, tests, security scans
- Build artifacts stored with commit hash
- Reproducible builds verified
- Auto-deploy to staging on main branch merge

**3. Internal Staging**
- Production-like environment for team testing
- Real relay nodes (3-5 nodes), real VPN nodes
- Testnet blockchains (Bitcoin testnet, Ethereum Sepolia)
- Updated automatically from main branch
- Team uses this for dogfooding Invisible

**4. External Beta (Phase 7)**
- Limited release to beta testers
- Separate infrastructure from internal staging
- Real mainnet blockchains (with warnings)
- Monitoring and crash reporting
- Opt-in for external testers

**5. Production (Phase 7)**
- Full production deployment
- Community-operated relay/VPN nodes
- Mainnet blockchains
- App store releases (iOS App Store, Google Play, F-Droid)
- Multiple regions, high availability

### Infrastructure Architecture

**Mobile App Deployment:**

**iOS**
- **Xcode Cloud** for builds
- **TestFlight** for internal testing (Phase 0+)
- **TestFlight** for external beta (Phase 7)
- **App Store** for production (Phase 7)
- **Reproducible builds**: Anyone can verify binary matches source
- **Automatic updates** via App Store

**Android**
- **GitHub Actions** for builds
- **Internal distribution** via Firebase App Distribution (Phase 0+)
- **Google Play Internal Testing** (Phase 6)
- **Google Play Beta** (Phase 7)
- **Google Play Production** (Phase 7)
- **F-Droid** for open source distribution (Phase 7)
- **APK direct download** for users in restricted countries
- **Reproducible builds** verified

**Relay Node Deployment:**

**Infrastructure as Code**
- **Terraform** for cloud infrastructure
- **Ansible** for node configuration
- Support AWS, GCP, Azure, DigitalOcean, Vultr, Linode
- Support bare metal deployment

**One-Click Deploy for Community Operators**
- **Docker image** pre-configured relay node
- **Docker Compose** template for easy setup
- **Kubernetes Helm chart** for production deployments
- **Systemd service** for bare metal
- Configuration via environment variables
- Automated updates (opt-in)

**Node Requirements**
- Minimum: 2 CPU, 4GB RAM, 100GB SSD, 1Gbps network
- Recommended: 4 CPU, 8GB RAM, 500GB SSD, 10Gbps network
- RAM-only operation (no disk writes)
- Automatic health monitoring
- Geographic diversity tracking

**VPN Node Deployment:**

**WireGuard VPN Nodes**
- Same deployment process as relay nodes
- Pre-configured WireGuard setup
- Automated key rotation
- Multiple exit locations (50+ initially)
- Load balancing across regions
- No logs, RAM-only

**Geographic Distribution**
- Minimum 3 continents
- Avoid Five Eyes concentration
- Jurisdiction diversity (no more than 20% in same country)
- Automatic endpoint selection for clients

### CI/CD Pipeline

**On every commit to feature branch:**
1. **Lint**: rustfmt, clippy, Flutter analyzer
2. **Unit Tests**: Full test suite (<5 min)
3. **Security Scan**: cargo-audit, dependency checks
4. **Build**: Debug build for all platforms

**On every PR:**
1. All of the above, plus:
2. **Integration Tests**: Full integration test suite (~15 min)
3. **Fuzzing**: Short fuzzing run (10 min)
4. **Performance Tests**: Benchmark suite, check for regressions
5. **Code Review**: Required approvals based on code type
6. **Security Review**: For crypto/wallet/network code

**On merge to main branch:**
1. All of the above, plus:
2. **Build Release Artifacts**:
   - iOS IPA (signed for TestFlight)
   - Android APK + AAB (signed)
   - Relay node Docker image
   - VPN node Docker image
3. **Deploy to Internal Staging**: Automatic deployment
4. **Run Smoke Tests**: Critical user flows on staging
5. **Tag Release**: Create git tag with version

**On git tag (release):**
1. **Build Production Artifacts**: Reproducible builds
2. **Sign Artifacts**: Code signing for all platforms
3. **Upload to Distribution**:
   - iOS → TestFlight or App Store
   - Android → Play Console or F-Droid
   - Docker → Docker Hub
4. **Create Release Notes**: Auto-generated changelog
5. **Notify Team**: Slack/Discord notification

### Monitoring & Observability

**Application Monitoring**
- **Sentry** for crash reporting (privacy-preserving mode)
- **Custom metrics** for privacy features (relay success rate, VPN uptime)
- **No user tracking**: Zero analytics on user behavior
- **Performance monitoring**: Client-side latency, battery usage

**Infrastructure Monitoring**
- **Prometheus + Grafana** for metrics
- **Alerting**: PagerDuty for critical issues
- **Uptime monitoring**: Pingdom or UptimeRobot for relay/VPN nodes
- **Log aggregation**: Loki (logs from nodes only, no user data)

**Privacy-Preserving Metrics**
- Network health (relay uptime, latency distribution)
- Success rates (message delivery, VPN connection)
- No user identifiers in any metrics
- Aggregate statistics only

### Deployment Runbooks

**Mobile App Release**
1. Merge release branch to main
2. Tag release (triggers CI/CD)
3. Review builds in TestFlight/Internal Testing
4. QA approval (smoke tests on real devices)
5. Submit to App Store/Play Store
6. Monitor crash reports for 48 hours
7. Rollout to 100% if no critical issues

**Relay Node Deployment**
1. Build Docker image (automated)
2. Deploy to staging relay cluster
3. Monitor for 24 hours
4. Deploy to 10% of production relays (canary)
5. Monitor for 48 hours
6. Rollout to 100% if healthy
7. Rollback procedure: `kubectl rollout undo`

**Incident Response**
1. Alert received (PagerDuty)
2. On-call engineer triages (severity assessment)
3. Create incident channel (Slack)
4. Investigate and mitigate
5. Post-mortem document (within 48 hours)
6. Action items tracked and completed

### Easy Deployment Promise

**For end users:**
- Download from App Store: 2 taps
- Onboarding: <2 minutes
- First message sent: <5 minutes from download

**For community node operators:**
- Setup time: <15 minutes
- One command: `docker-compose up -d`
- Automated updates (opt-in)
- Monitoring dashboard included

**For developers:**
- Local dev setup: <10 minutes (`./scripts/dev-setup.sh`)
- PR to staging: <30 minutes (automated)
- Staging to production: <2 hours (with approvals)

---

## Communication & Collaboration

**Core Principle: Transparent communication, autonomous teams, clear ownership**

### Communication Channels

**Synchronous Communication**

**Daily Standups (15 min per team)**
- **Time**: 9:30 AM (each team picks their time)
- **Format**: What I did, what I'm doing, blockers
- **Rule**: No problem-solving in standup (take it offline)
- **Attendance**: Team members only (not company-wide)

**Weekly Cross-Team Sync (1 hour)**
- **Time**: Fridays 2pm
- **Attendees**: All team leads + architects
- **Agenda**:
  - Sprint demos from each team
  - Cross-team dependencies discussion
  - Upcoming blockers
  - Architecture decisions needed
- **Output**: Action items with owners

**Design/Eng Sync (15 min daily)**
- **Time**: 10:30 AM
- **Attendees**: Design team + relevant eng teams
- **Purpose**: Catch implementation issues early
- **Format**: Quick design QA, questions, clarifications

**Security Review Sessions (2x per week)**
- **Time**: Tuesday & Thursday 3pm
- **Duration**: 1-2 hours
- **Format**: Code walkthrough for crypto/wallet/network changes
- **Attendees**: Security team + code authors

**Asynchronous Communication**

**Slack/Discord Organization**
- `#general` - Company announcements
- `#engineering` - General engineering discussion
- `#design` - Design team coordination
- `#security` - Security findings and discussions
- `#phase-N` - Per-phase channels (archive when phase completes)
- `#team-crypto` - Crypto team
- `#team-mobile` - Mobile team
- `#team-backend` - Backend team
- `#team-wallet` - Wallet team
- `#incidents` - Production issues
- `#releases` - Release notifications

**Documentation**

**Living Documentation (always up-to-date)**
- `/docs/architecture/` - Architecture decision records (ADRs)
- `/docs/api/` - API documentation (auto-generated)
- `/docs/security/` - Threat models, security reviews
- `/docs/design/` - Design system, UI specifications
- `/docs/onboarding/` - New team member guides
- `/docs/runbooks/` - Operational procedures

**Decision Records (ADRs)**
- Document significant technical decisions
- Format: Context, Decision, Consequences
- Stored in git, versioned with code
- Reviewed in architecture sync

### Meeting Cadence

**Sprint Ceremonies (every 2 weeks)**
- **Sprint Planning** (Monday Week 1, 4 hours)
- **Sprint Demo** (Thursday Week 2, 1 hour)
- **Sprint Retrospective** (Friday Week 2, 1 hour)

**Design Reviews (weekly)**
- **Design Critique** (Wednesdays, 2 hours)
- **Design/Eng Handoff** (Fridays, 1 hour)
- **User Testing Readouts** (Fridays, 30 min)

**Security & Architecture (weekly)**
- **Threat Modeling** (Mondays, 1 hour)
- **Security Code Review** (Tuesday/Thursday, 1-2 hours each)
- **Architecture Sync** (Fridays, 1 hour)

**No-Meeting Time**
- **Deep Work Blocks**: Tuesdays & Thursdays 10am-2pm
- No meetings scheduled during deep work blocks
- Engineers can focus on complex problems uninterrupted

### Code Review Process

**Pull Request Guidelines**

**PR Size**
- Target: <500 lines changed
- Maximum: 1000 lines (break up larger changes)
- Exception: Generated code (must note in PR description)

**PR Description Must Include**
- What: What does this change do?
- Why: Why is this change needed?
- How: Brief overview of approach
- Testing: How was this tested?
- Screenshots: For UI changes
- Security: Any security implications?

**Review Requirements (varies by code type)**

**Regular Code**
- 1 engineer approval required
- CI must pass
- No merge conflicts

**Security-Sensitive Code (crypto/wallet/auth)**
- 2 security engineer approvals required
- 1 domain expert approval required
- Security checklist completed
- CI must pass

**UI Code**
- 1 engineer approval required
- 1 designer approval required
- Visual regression tests pass
- Accessibility checklist completed

**Review SLA**
- First review within 4 hours (during work hours)
- Approval or feedback within 24 hours
- No PR should wait >48 hours

**Review Etiquette**
- ✅ Be constructive, not critical
- ✅ Ask questions, don't make demands
- ✅ Approve if "good enough", don't nitpick
- ✅ Distinguish between "must fix" and "nice to have"
- ❌ Don't bikeshed (argue about trivial style issues)

### Knowledge Sharing

**Weekly Knowledge Shares (Fridays 3pm, 30 min)**
- Rotating presentations from team members
- Topics: New techniques, lessons learned, interesting problems solved
- Optional attendance but encouraged
- Recorded for async viewing

**Pair Programming**
- **Required**: All crypto code must be pair programmed
- **Encouraged**: Complex algorithms, new domains
- **Tools**: VS Code Live Share, Tuple, Screen sharing

**Documentation Culture**
- Every feature needs documentation before merging
- Update docs when code changes
- README files in every directory
- Code comments explain "why", not "what"

**Onboarding Buddies**
- New team members get assigned buddy
- Buddy helps with setup, answers questions, reviews first PRs
- Weekly check-ins for first month

### Conflict Resolution

**Technical Disagreements**
1. Discuss in relevant channel (Slack/Discord)
2. If no consensus in 24 hours → schedule sync meeting
3. If still no consensus → escalate to Tech Lead
4. Tech Lead makes final call, documents decision in ADR

**Process Issues**
1. Raise in sprint retrospective
2. Team discusses and votes on changes
3. Try new process for 2 sprints
4. Evaluate in retro, keep or revert

**Cross-Team Blockers**
1. Raise in weekly cross-team sync
2. Team leads negotiate solution
3. If no resolution → escalate to Product Lead
4. Document resolution and update plans

### Remote/Hybrid Work

**Work Schedule**
- **Core hours**: 10am-3pm (local time) - overlap time
- **Flexible**: Teams can adjust based on timezone needs
- **Async-first**: Assume not everyone is online at once

**Time Zones**
- Document each team member's timezone
- Schedule meetings during overlap hours
- Record important meetings for async viewing
- Use async communication (docs, Slack) when possible

**In-Person (if applicable)**
- Optional quarterly team gatherings
- Sprint planning can be in-person if preferred
- Design sprints benefit from in-person collaboration

### Tools & Platforms

**Development**
- **Git**: GitHub for code hosting
- **CI/CD**: GitHub Actions
- **Code Review**: GitHub PRs
- **IDE**: VS Code (recommended), any preferred

**Design**
- **Design Tool**: Figma (single source of truth)
- **Prototyping**: Figma, ProtoPie
- **User Testing**: UserTesting.com, Maze

**Communication**
- **Chat**: Slack or Discord
- **Video**: Zoom or Google Meet
- **Async Video**: Loom for demos/walkthroughs

**Project Management**
- **Sprints**: Jira or Linear
- **Roadmap**: ProductBoard or Notion
- **Documentation**: Notion or GitBook

**Monitoring**
- **Errors**: Sentry
- **Metrics**: Grafana
- **Incidents**: PagerDuty

---

## Technology Stack & Tools

*(Enhanced from existing CLAUDE.md content)*

### Core Technologies

| Layer | Technology | Why |
|-------|-----------|-----|
| Crypto core | Rust (libsignal-protocol, ring, ML-KEM) | Memory safety, performance, no GC |
| Mobile clients | Flutter/Dart | Cross-platform, single codebase |
| Relay/mix nodes | Rust (tokio async) | Performance, safety, small binary |
| VPN protocol | WireGuard | Audited, minimal, fast |
| Packet format | Sphinx (2KB uniform) | Proven mixnet packet format |
| Local storage | SQLCipher + Argon2id | Encrypted SQLite, memory-hard KDF |
| Networking | libp2p | Proven P2P networking stack |
| Transports | obfs5, uTLS | DPI resistance |
| Calls | WebRTC + custom SRTP | Standard media + custom key exchange |
| Wallet | monero-rs, rust-bitcoin, ethers-rs | Native Rust chain libraries |
| Swaps | COMIT HTLC | Proven atomic swap infrastructure |
| DeFi | WalletConnect v2 | Standard wallet connection |
| 2FA | TOTP (RFC 6238) + FIDO2 | Standard, proven second factors |
| Build | Reproducible builds | Verifiable binaries, trust through transparency |

### Development Tools

- **Version Control**: Git, GitHub
- **CI/CD**: GitHub Actions
- **IDE**: VS Code (recommended), any editor
- **Linting**: rustfmt, clippy, Flutter analyzer
- **Testing**: cargo test, criterion (benchmarks), proptest (property testing)

### Design Tools

- **Design**: Figma (single source of truth)
- **Prototyping**: Figma, ProtoPie
- **User Testing**: UserTesting.com, Maze
- **Accessibility**: WAVE, Axe DevTools

### Communication Tools

- **Chat**: Slack or Discord
- **Video**: Zoom or Google Meet
- **Async**: Loom for demos and walkthroughs
- **Docs**: Notion or GitBook

### Project Management Tools

- **Sprints**: Jira or Linear
- **Roadmap**: ProductBoard or Notion
- **Design Handoff**: Figma + Zeplin

### Monitoring Tools

- **Application**: Sentry (crash reporting)
- **Infrastructure**: Prometheus + Grafana
- **Uptime**: Pingdom or UptimeRobot
- **Incidents**: PagerDuty
- **Logs**: Loki (infrastructure only, no user data)

---

## Phase 0: Foundation (Weeks 1-8)
**Goal:** Core cryptographic primitives and local-first architecture.

| Milestone | Description | Epic |
|-----------|-------------|------|
| M0.1 | Rust crypto core — Signal Protocol Double Ratchet, X3DH, Ed25519 | [epic-00](spec/epics/epic-00-foundation.md) |
| M0.2 | Post-quantum key exchange (PQXDH via ML-KEM + X25519) | [epic-00](spec/epics/epic-00-foundation.md) |
| M0.3 | SQLCipher encrypted local storage + Argon2id KDF | [epic-00](spec/epics/epic-00-foundation.md) |
| M0.4 | Device identity generation + per-conversation key management | [epic-00](spec/epics/epic-00-foundation.md) |
| M0.5 | Daily master key rotation system | [epic-00](spec/epics/epic-00-foundation.md) |

**Deliverable:** Crypto library with full test coverage, encrypted local DB, identity system.

### Team Composition - Phase 0

**Active Teams:**
- **Crypto & Core Team:** 5 engineers (100% allocation)
- **Mobile Team:** 4 engineers (40% allocation - project setup, architecture)
- **Backend Team:** 3 engineers (30% allocation - relay node architecture planning)
- **Design Team:** 4 designers (25% allocation - design system foundation, onboarding flows)
- **Security Team:** 2 engineers (50% allocation - threat modeling, crypto review)
- **DevOps Team:** 3 engineers (75% allocation - CI/CD setup, infrastructure planning)

**Total: 21 people active in Phase 0**

### Sprint Breakdown - Phase 0 (4 Sprints)

**Sprint 0.1 (Weeks 1-2): Project Setup + Crypto Core Foundation**
- **Sprint Goal:** Setup development infrastructure and begin Signal Protocol implementation
- **Key Deliverables:** X3DH key agreement, CI/CD pipeline, Flutter scaffolding, design system foundations
- **Success Criteria:** X3DH passes NIST test vectors, CI running, local dev environment working

**Sprint 0.2 (Weeks 3-4): Double Ratchet + Post-Quantum**
- **Sprint Goal:** Complete Signal Protocol Double Ratchet and add post-quantum protection
- **Key Deliverables:** Double Ratchet implementation, PQXDH hybrid key exchange, FFI bindings
- **Success Criteria:** End-to-end encrypted messages working, all crypto operations constant-time

**Sprint 0.3 (Weeks 5-6): Encrypted Storage + Device Identity**
- **Sprint Goal:** Implement SQLCipher storage and device identity generation
- **Key Deliverables:** Encrypted database, device key generation, 2FA integration
- **Success Criteria:** Data encrypted at rest, 2FA required for app access

**Sprint 0.4 (Weeks 7-8): Daily Key Rotation + Phase 0 Completion**
- **Sprint Goal:** Implement daily master key rotation and complete Phase 0
- **Key Deliverables:** Key rotation system, comprehensive test suite, phase retrospective
- **Success Criteria:** All Phase 0 milestones complete, external mini-audit scheduled

**Note:** Full sprint breakdowns for Phase 1-7 will be added incrementally as each phase approaches, following the same template established in Phase 0.

---

## Phase 1: Core Messaging (Weeks 9-16)

**Goal:** 1-on-1 encrypted messaging with zero-identifier contacts.

| Milestone | Description | Epic |
|-----------|-------------|------|
| M1.1 | 1-on-1 E2EE messaging (Double Ratchet) | [epic-01](spec/epics/epic-01-messaging.md) |
| M1.2 | Disappearing messages (configurable 24h-90d) | [epic-01](spec/epics/epic-01-messaging.md) |
| M1.3 | QR-code and one-time link contact exchange | [epic-02](spec/epics/epic-02-contacts.md) |
| M1.4 | Pairwise anonymous identifiers | [epic-02](spec/epics/epic-02-contacts.md) |
| M1.5 | Key verification (safety numbers) | [epic-02](spec/epics/epic-02-contacts.md) |
| M1.6 | Mandatory 2FA + biometric gating | [epic-16](spec/epics/epic-16-access-control.md) |

**Deliverable:** Working encrypted messenger with contact exchange, auto-purge, mandatory 2FA.

---

## Phase 2: The Scrambler (Weeks 17-28)

**Goal:** Full 7-layer network obfuscation system.

| Milestone | Description | Epic |
|-----------|-------------|------|
| M2.1 | Sphinx packet format implementation (2KB uniform) | [epic-03](spec/epics/epic-03-scrambler.md) |
| M2.2 | Layer 1: Shamir's Secret Sharing message fragmentation | [epic-03](spec/epics/epic-03-scrambler.md) |
| M2.3 | Layer 2: 5-layer mixnet (Loopix architecture) | [epic-03](spec/epics/epic-03-scrambler.md) |
| M2.4 | Layer 3: Cover traffic (constant-rate Sphinx loop traffic) | [epic-03](spec/epics/epic-03-scrambler.md) |
| M2.5 | Layer 4: Jurisdiction routing (multi-jurisdiction path enforcement) | [epic-03](spec/epics/epic-03-scrambler.md) |
| M2.6 | Layer 5: Protocol camouflage (obfs5, uTLS, domain fronting) | [epic-03](spec/epics/epic-03-scrambler.md) |
| M2.7 | Layer 6: Dead drop architecture (anonymous relay mailboxes) | [epic-03](spec/epics/epic-03-scrambler.md) |
| M2.8 | Layer 7: Temporal scrambling (Poisson-distributed delays) | [epic-03](spec/epics/epic-03-scrambler.md) |
| M2.9 | Relay node software (RAM-only processing) | [epic-14](spec/epics/epic-14-relay-nodes.md) |

**Deliverable:** Complete Scrambler stack routing all messages through mixnet with cover traffic.

---

## Phase 3: Ghost VPN + Hardening (Weeks 29-36)

**Goal:** Mandatory VPN layer, cellular metadata protection, anti-forensics, device lockdown.

| Milestone | Description | Epic/Doc |
|-----------|-------------|------|
| M3.1 | Ghost VPN: WireGuard integration, ephemeral keys | [epic-09](spec/epics/epic-09-ghost-vpn.md) |
| M3.2 | Random global endpoint selection (50+ nodes) | [epic-09](spec/epics/epic-09-ghost-vpn.md) |
| M3.3 | Session timeout + auto-lock + max lifetime | [epic-09](spec/epics/epic-09-ghost-vpn.md) |
| M3.4 | **Network Privacy Mode: WiFi-Only mode (cellular data blocking)** | [network-privacy](spec/architecture/network-privacy-mode.md) |
| M3.5 | **Network Privacy Mode: eSIM rotation manager (auto-rotate profiles)** | [network-privacy](spec/architecture/network-privacy-mode.md) |
| M3.6 | **Network Privacy Mode: MAC address randomization enforcement** | [network-privacy](spec/architecture/network-privacy-mode.md) |
| M3.7 | **Network Privacy Mode: Airplane Mode + WiFi quick toggle** | [network-privacy](spec/architecture/network-privacy-mode.md) |
| M3.8 | **Network Privacy Mode: UI (settings screen + status bar toggle)** | [network-privacy](spec/architecture/network-privacy-mode.md) |
| M3.9 | Anti-forensics (no thumbnails, no clipboard, blank task switcher) | [epic-08](spec/epics/epic-08-hardening.md) |
| M3.10 | Screen capture prevention (FLAG_SECURE, capture detection) | [epic-08](spec/epics/epic-08-hardening.md) |
| M3.11 | Secure keyboard (optional built-in, no keystroke logging) | [epic-08](spec/epics/epic-08-hardening.md) |
| M3.12 | Duress PIN + panic gesture + remote wipe | [epic-16](spec/epics/epic-16-access-control.md) |

**Deliverable:** App launches only through VPN, cellular metadata protection (WiFi-only/eSIM rotation/airplane mode), full anti-forensic hardening, panic features.

---

## Phase 4: Groups + Media + Calls (Weeks 37-48)

**Goal:** Full-featured secure communication + admin dashboard for power users.

| Milestone | Description | Epic/Doc |
|-----------|-------------|------|
| M4.1 | MLS-based group messaging (RFC 9420) | [epic-04](spec/epics/epic-04-groups.md) |
| M4.2 | Encrypted member lists | [epic-04](spec/epics/epic-04-groups.md) |
| M4.3 | Encrypted file/image/voice transfer | [epic-05](spec/epics/epic-05-media.md) |
| M4.4 | RAM-based media viewer (no disk writes) | [epic-05](spec/epics/epic-05-media.md) |
| M4.5 | E2EE voice/video calls (WebRTC + custom SRTP) | [epic-06](spec/epics/epic-06-calls.md) |
| M4.6 | Burn Rooms — self-destructing chats | [epic-15](spec/epics/epic-15-burn-rooms.md) |
| M4.7 | **Admin Dashboard: UI framework (tabs, navigation, responsive)** | [admin-dashboard](spec/architecture/admin-dashboard.md) |
| M4.8 | **Admin Dashboard: Configuration tab (service toggles, privacy presets)** | [admin-dashboard](spec/architecture/admin-dashboard.md) |
| M4.9 | **Admin Dashboard: Status dashboard (real-time monitoring, health checks)** | [admin-dashboard](spec/architecture/admin-dashboard.md) |
| M4.10 | **Admin Dashboard: Metrics collection and visualization (graphs, stats)** | [admin-dashboard](spec/architecture/admin-dashboard.md) |
| M4.11 | **Admin Dashboard: Activity logging system (events, filters, export)** | [admin-dashboard](spec/architecture/admin-dashboard.md) |
| M4.12 | **Admin Dashboard: Message send indicator (layer-by-layer progress)** | [admin-dashboard](spec/architecture/admin-dashboard.md) |

**Deliverable:** Group messaging, media sharing, voice/video calls, burn rooms, comprehensive admin dashboard with full visibility and control over all privacy features.

---

## Phase 5: Shadow Wallet + Crypto (Weeks 49-64)

**Goal:** Built-in privacy-first financial layer with fortress-grade security.

### Phase 5a: Wallet Foundation + Critical Hardening (Weeks 49-52)

| Milestone | Description | Epic/Doc |
|-----------|-------------|------|
| M5.1 | Non-custodial wallet generation + key management | [epic-10](spec/epics/epic-10-shadow-wallet.md) |
| M5.2 | **Hardware security module integration (iOS Secure Enclave, Android StrongBox)** | [hardening](spec/architecture/shadow-wallet-hardening.md) |
| M5.3 | **Transaction verification layer (address, amount, fee, replay protection)** | [hardening](spec/architecture/shadow-wallet-hardening.md) |
| M5.4 | **Enhanced memory protection (mlock, memory encryption, secure zeroing)** | [hardening](spec/architecture/shadow-wallet-hardening.md) |

**Deliverable:** Wallet foundation with hardware-protected keys and comprehensive transaction verification.

### Phase 5b: Privacy Coins + User Protection (Weeks 53-56)

| Milestone | Description | Epic/Doc |
|-----------|-------------|------|
| M5.5 | Monero integration (ring signatures, stealth addresses, subaddresses) | [epic-11](spec/epics/epic-11-privacy-coins.md) |
| M5.6 | Zcash shielded + Bitcoin privacy (CoinJoin/PayJoin, Silent Payments) | [epic-11](spec/epics/epic-11-privacy-coins.md) |
| M5.7 | In-chat payments (send/receive/split, timing decorrelation) | [epic-10](spec/epics/epic-10-shadow-wallet.md) |
| M5.8 | **Anti-phishing protection (identity verification, suspicious patterns)** | [hardening](spec/architecture/shadow-wallet-hardening.md) |
| M5.9 | **Smart contract security (approval limits, simulation, verification)** | [hardening](spec/architecture/shadow-wallet-hardening.md) |
| M5.10 | **Multi-signature wallet coordination via messenger** | [hardening](spec/architecture/shadow-wallet-hardening.md) |

**Deliverable:** Privacy coins integrated with user-facing security protections.

### Phase 5c: Advanced Features + Backup (Weeks 57-60)

| Milestone | Description | Epic/Doc |
|-----------|-------------|------|
| M5.11 | Phantom Swap — atomic cross-chain swaps (HTLC) | [epic-12](spec/epics/epic-12-phantom-swap.md) |
| M5.12 | XMR Hop for maximum unlinkability | [epic-12](spec/epics/epic-12-phantom-swap.md) |
| M5.13 | DeFi proxy — WalletConnect v2, anonymous RPC | [epic-13](spec/epics/epic-13-defi-proxy.md) |
| M5.14 | **Shamir seed backup (SLIP-39) + social recovery** | [hardening](spec/architecture/shadow-wallet-hardening.md) |
| M5.15 | **Privacy coin enhancements (Monero view keys, auto-CoinJoin, Zcash enforcement)** | [hardening](spec/architecture/shadow-wallet-hardening.md) |
| M5.16 | **Broadcast verification + fallback mechanism** | [hardening](spec/architecture/shadow-wallet-hardening.md) |

**Deliverable:** Full crypto wallet with atomic swaps, DeFi access, and advanced backup options.

### Phase 5d: Defense in Depth + Testing (Weeks 61-64)

| Milestone | Description | Epic/Doc |
|-----------|-------------|------|
| M5.17 | Ethereum + stablecoins (ZK-rollup privacy layers) | [epic-10](spec/epics/epic-10-shadow-wallet.md) |
| M5.18 | **Side-channel attack mitigation (constant-time ops, cache protection)** | [hardening](spec/architecture/shadow-wallet-hardening.md) |
| M5.19 | **Comprehensive wallet security testing** | [hardening](spec/architecture/shadow-wallet-hardening.md) |
| M5.20 | **Wallet-specific security audit preparation** | [hardening](spec/architecture/shadow-wallet-hardening.md) |

**Deliverable:** Fortress-grade wallet with defense-in-depth security, ready for audit.

**Phase 5 Complete Deliverable:** Full crypto wallet with privacy coins, atomic swaps, anonymous DeFi access, hardware-protected keys, comprehensive transaction verification, anti-phishing protection, smart contract security, Shamir backup, and side-channel protections.

---

## Phase 6: Mesh + Community Nodes (Weeks 65-76)

**Goal:** Offline capability and decentralized infrastructure.

| Milestone | Description | Epic |
|-----------|-------------|------|
| M6.1 | Bluetooth/WiFi Direct mesh messaging | [epic-07](spec/epics/epic-07-mesh.md) |
| M6.2 | Store-and-forward for offline contacts | [epic-07](spec/epics/epic-07-mesh.md) |
| M6.3 | Community relay/mix/VPN node operation | [epic-14](spec/epics/epic-14-relay-nodes.md) |
| M6.4 | Warrant canary system | [epic-14](spec/epics/epic-14-relay-nodes.md) |
| M6.5 | Node health monitoring + auto-deprioritization | [epic-14](spec/epics/epic-14-relay-nodes.md) |

**Deliverable:** Mesh networking, community-operated infrastructure, fully decentralized.

---

## Phase 7: Audit + Launch (Weeks 77-88)

**Goal:** Security audit, reproducible builds, public launch.

| Milestone | Description |
|-----------|-------------|
| M7.1 | Independent cryptographic audit (e.g., NCC Group, Trail of Bits) |
| M7.2 | Reproducible build pipeline for verifiable binaries |
| M7.3 | Open-source release (all client + relay code) |
| M7.4 | Public beta launch |
| M7.5 | Bug bounty program |
| M7.6 | General availability |

**Deliverable:** Audited, open-source, publicly available Invisible messenger.

---

## Dependencies

```
Phase 0 (Foundation) ──→ Phase 1 (Messaging) ──→ Phase 2 (Scrambler)
                                    │                      │
                                    ▼                      ▼
                          Phase 4 (Groups/Media)   Phase 3 (VPN/Hardening)
                                    │                      │
                                    └──────────┬───────────┘
                                               ▼
                                    Phase 5 (Shadow Wallet)
                                    [16 weeks - includes hardening]
                                               │
                                               ▼
                                    Phase 6 (Mesh/Nodes)
                                               │
                                               ▼
                                    Phase 7 (Audit/Launch)
```

---

## Success Metrics & Monitoring

**How we measure progress and health across engineering, design, security, and deployment**

### Engineering Metrics

**Sprint-Level**
- Milestone delivery on time (target: 90%+)
- Test coverage (target: 85% overall, 100% crypto/wallet)
- Code review turnaround (target: <24 hours)
- CI/CD pipeline success rate (target: 95%+)

**Phase-Level**
- All milestones delivered
- No critical bugs in backlog
- Performance benchmarks met
- External security audit passed

### Design Metrics

**User Testing Success**
- Task completion rate (target: 80%+)
- Time to complete critical flows (target: <30 seconds to understand)
- User satisfaction scores
- Accessibility audit compliance (WCAG 2.1 AA)

**Design QA**
- Implementation matches design specs (100%)
- No major usability issues identified
- All edge cases handled

### Security Metrics

**Continuous Validation**
- Security code reviews completed (100% of crypto/wallet code)
- Vulnerability findings remediated (100% critical, 90% high)
- Fuzzing uptime (target: 95%+ of time)
- External audit findings per phase (target: <5 critical)

**Test Coverage**
- Crypto code: 100% branch coverage
- Wallet code: 100% branch coverage
- Network code: 90% coverage
- Overall: 85%+ coverage

### Deployment Metrics

**CI/CD Performance**
- PR to staging time (target: <30 minutes)
- Staging to production time (target: <2 hours with approvals)
- Build success rate (target: 95%+)
- Deployment rollbacks (target: <5% of deployments)

**Infrastructure Health**
- Relay node uptime (target: 99%+)
- VPN node uptime (target: 99.5%+)
- Message delivery success rate (target: 99%+)
- Average message latency (target: <45 seconds for High privacy preset)

### Network Health Metrics

**Decentralization**
- Number of community relay nodes (target: 100+ by Phase 6)
- Geographic distribution (target: 5+ continents)
- Jurisdiction diversity (target: <20% in any single country)

**Performance**
- Network latency (P50, P95, P99)
- Relay throughput (target: 10k messages/sec per node)
- Cover traffic efficiency
- Mixnet path diversity

---

## Risk Management & Mitigation

**Identifying and mitigating risks across technical, team, security, and deployment domains**

### Technical Risks

**Cryptography Complexity**
- **Risk:** Post-quantum crypto library immaturity, integration complexity
- **Mitigation:** X25519-only fallback, extensive testing with NIST vectors, external crypto audit

**Performance Bottlenecks**
- **Risk:** 7-layer Scrambler introduces high latency, poor mobile performance
- **Mitigation:** Benchmark every sprint, optimize hot paths, privacy level presets for user control

**Platform Compatibility**
- **Risk:** iOS/Android platform differences, hardware security module variations
- **Mitigation:** Platform specialists on mobile team, extensive device testing, graceful degradation

**Blockchain Integration**
- **Risk:** Monero/Zcash library instability, testnet unreliability
- **Mitigation:** Wallet team starts in Phase 2 planning, testnets + mainnet testing, fallback to mock services

### Team Risks

**Scaling Challenges**
- **Risk:** Growing from 29 to 41 people, communication overhead, process breakdown
- **Mitigation:** Clear team structure, autonomous teams, strong leads, documented processes

**Knowledge Silos**
- **Risk:** Critical knowledge concentrated in 1-2 people (crypto, wallet, mixnet)
- **Mitigation:** Pair programming required for crypto, documentation culture, knowledge shares

**Burnout**
- **Risk:** 88-week timeline, high-pressure security work, continuous red team attacks
- **Mitigation:** No-meeting deep work blocks, flexible schedules, phase-end retrospectives

### Security Risks

**Undiscovered Vulnerabilities**
- **Risk:** Critical security flaw discovered late in development or after launch
- **Mitigation:** Security in every sprint, red team attacks, external audits, bug bounty program

**Side-Channel Attacks**
- **Risk:** Timing attacks, cache-timing attacks, power analysis on crypto operations
- **Mitigation:** Constant-time operations, valgrind verification, formal verification for critical paths

**Supply Chain Attacks**
- **Risk:** Compromised dependencies, malicious crates, backdoors in libraries
- **Mitigation:** cargo-audit in CI, dependency pinning, minimize dependencies, audit critical deps

### Deployment Risks

**App Store Rejections**
- **Risk:** Apple/Google reject app for privacy features, VPN usage, crypto wallet
- **Mitigation:** Early TestFlight submission, compliance review, F-Droid + direct APK as fallback

**Node Operator Adoption**
- **Risk:** Insufficient community relay nodes, centralization risk
- **Mitigation:** One-click deployment, economic incentives, early community building

**Regulatory Challenges**
- **Risk:** Legal challenges in certain jurisdictions, forced shutdown
- **Mitigation:** Decentralized architecture, reproducible builds, open source, jurisdiction diversity

---

## Shadow Wallet Security Hardening

**Phase 5 now integrates comprehensive wallet security hardening throughout development.**

The Shadow Wallet already implements the full 7-layer Scrambler protection for all financial operations (transaction fragmentation, financial cover traffic, multi-node broadcast, temporal scrambling, network isolation). Phase 5 adds critical application-layer hardening to match messenger security:

### Critical (P0) - Weeks 49-52
- **Hardware Security Module Integration** - Keys in Secure Enclave/StrongBox, never in app memory
- **Transaction Verification Layer** - Prevents clipboard hijacking, address poisoning, replay attacks
- **Enhanced Memory Protection** - mlock, memory encryption, secure zeroing

### High Priority (P1) - Weeks 53-56
- **Anti-Phishing Protection** - Identity verification, suspicious pattern detection
- **Smart Contract Security** - Block unlimited approvals, transaction simulation, contract verification
- **Multi-Signature Coordination** - Secure co-signer coordination via messenger

### Medium Priority (P2) - Weeks 57-60
- **Seed Backup Hardening** - Shamir Secret Sharing (SLIP-39), social recovery
- **Privacy Coin Enhancements** - Monero view keys, auto-CoinJoin, Zcash shielded enforcement
- **Broadcast Verification** - Confirm tx in mempool, fallback mechanism

### Defense in Depth (P3) - Weeks 61-64
- **Side-Channel Mitigation** - Constant-time ops, cache-timing protection
- **Security Testing** - Comprehensive wallet security validation
- **Audit Preparation** - Documentation and threat modeling

### Documentation

- **[WALLET-HARDENING-SUMMARY.md](WALLET-HARDENING-SUMMARY.md)** - Executive summary
- **[SHADOW-WALLET-SECURITY-ANALYSIS.md](SHADOW-WALLET-SECURITY-ANALYSIS.md)** - Detailed gap analysis
- **[spec/architecture/shadow-wallet-hardening.md](spec/architecture/shadow-wallet-hardening.md)** - Technical specifications

**Result:** Wallet security will match messenger security at all layers - both network-level (already complete via Scrambler) and application-level (added through hardening).

---

## Network Privacy Mode (Cellular Metadata Protection)

**Phase 3 now includes protection against cellular network metadata leakage.**

Even with Ghost VPN encrypting all traffic, cellular carriers can still track users via IMSI/IMEI identifiers, phone numbers, and cell tower triangulation. Network Privacy Mode closes this gap with multiple protection strategies:

### The Problem

**What cellular carriers can see (even with VPN):**
- IMSI (SIM card ID) - Persistent tracking across sessions
- IMEI (Hardware ID) - Survives SIM changes
- Phone number - Direct link to real identity
- Cell tower location - Continuous physical tracking
- VPN usage fingerprinting - Identifies privacy tool users

### The Solutions

**1. WiFi-Only Mode (M3.4)**
- Disables cellular data completely
- Only allows traffic over WiFi connections
- Cellular carrier sees NO data traffic
- Prevents VPN usage fingerprinting
- Can use public WiFi for additional anonymity

**2. eSIM Rotation (M3.5)**
- Automatically rotates between multiple eSIM profiles
- Each rotation = new IMSI + new phone number + new carrier
- Rotation strategies: Per session, daily, weekly, or manual
- Breaks long-term cellular tracking
- Works with anonymous eSIM providers (Silent.link, Hushed, Airalo)

**3. Airplane Mode + WiFi (M3.7)**
- Cellular radio completely disabled (no IMSI/IMEI broadcast)
- WiFi enabled for connectivity
- Zero cellular metadata exposure
- Cannot be tracked via cell towers or IMSI catchers

**4. MAC Address Randomization (M3.6)**
- Enforces WiFi MAC randomization (prevents access point tracking)
- Different MAC per WiFi network
- Breaks WiFi-based device tracking

### Threat Mitigation

| Threat | Ghost VPN Alone | + WiFi-Only | + eSIM Rotation | + Airplane+WiFi |
|--------|-----------------|-------------|-----------------|-----------------|
| Content surveillance | ✓ | ✓ | ✓ | ✓ |
| IP address leak | ✓ | ✓ | ✓ | ✓ |
| **IMSI tracking** | ⚠️ | ⚠️ | ✓ Rotated | ✓ Hidden |
| **Phone # linkage** | ⚠️ | ⚠️ | ✓ Rotated | ✓ Hidden |
| **Cell tower location** | ⚠️ | ⚠️ | ⚠️ | ✓ Hidden |
| **VPN fingerprinting** | ⚠️ | ✓ | ⚠️ | ✓ |
| **Long-term profiling** | ⚠️ | ✓ | ✓ | ✓ |

### Documentation

- **[NETWORK-PRIVACY-MODE-SUMMARY.md](NETWORK-PRIVACY-MODE-SUMMARY.md)** - Executive summary
- **[spec/architecture/network-privacy-mode.md](spec/architecture/network-privacy-mode.md)** - Technical specification

**Result:** Complete network anonymity from cellular carrier to internet destination - closes the cellular metadata gap that VPN alone cannot address.

---

## Admin Dashboard (Privacy Control Center)

**Phase 4 includes a comprehensive admin dashboard for power users.**

The Admin Dashboard provides complete transparency and control over every privacy protection in Invisible. Users can see exactly what's active, configure services independently, monitor real-time status, and understand performance metrics.

### Key Features

**1. Configuration Tab**
- Privacy level presets (Paranoid, High, Standard, Low) - one-tap switching
- Individual service toggles (enable/disable each layer independently)
- Fine-grained configuration for each protection layer
- Warnings when disabling critical services
- Save and apply changes with confirmation

**2. Status Dashboard Tab**
- Real-time monitoring of all active services
- Health status for each protection layer
- Connection details (VPN endpoint, network mode, etc.)
- Service metrics (latency, throughput, success rates)
- Auto-refresh every 2 seconds
- Overall system health indicator

**3. Privacy Level Presets**

| Level | Latency | Services | Use Case |
|-------|---------|----------|----------|
| **Paranoid** | 30-90s | ALL max settings | Journalists, activists, whistleblowers |
| **High** (default) | 5-45s | ALL balanced | Privacy-conscious users |
| **Standard** | 2-20s | Most enabled | Everyday secure messaging |
| **Low** | 1-8s | Essential only | Speed priority, low threat |

**4. Message Send Indicator**
- Real-time progress as message routes through Scrambler
- Layer-by-layer status (encrypted, fragmented, mixed, etc.)
- Shows active protections for each message
- Compact indicator in message bubble (tap to expand)
- Delivery time and path information

**5. Service Toggle System**
- Enable/disable individual services
- Core services locked (E2EE, Ghost VPN)
- Optional services configurable (cover traffic, eSIM, wallet features)
- Dependency warnings (e.g., dead drops require mixnet)
- Immediate visual feedback

**6. Metrics & Performance**
- Historical statistics (last 24h, 7d, 30d)
- Latency distribution graphs
- Success rates and uptime
- Bandwidth usage
- Jurisdiction distribution
- Export reports (CSV, JSON)

**7. Activity Logs**
- Detailed event log of all system activity
- Filterable by service, time, log level
- Shows VPN rotations, eSIM changes, message delivery, security events
- Export logs for auditing
- Privacy-preserving (no message content logged)

### Benefits

- ✅ **Complete transparency** - See exactly what protections are active
- ✅ **Full control** - Configure every service independently
- ✅ **Real-time monitoring** - Track system health and performance
- ✅ **Informed decisions** - Understand privacy vs speed tradeoffs
- ✅ **Power user features** - Advanced config, metrics, logs
- ✅ **Education** - Learn what each layer does

### User Experience

**Quick Access:**
- Status bar icon shows current privacy level
- Tap to open admin dashboard
- Swipe between tabs (Config, Status, Metrics, Logs)

**Message Indicator:**
```
Sending message...
✓ Encrypted (Double Ratchet + PQ)
✓ Fragmented (3-of-5 shares)
⏳ Routing through mixnet (Layer 2/5)
✓ Cover traffic active
...
Estimated delivery: 12-18s
```

**After sending:**
```
You: Hey, can we meet at 3pm?
🛡️ [7 layers] 14:32  ← Tap for details
```

### Documentation

- **[ADMIN-DASHBOARD-SUMMARY.md](ADMIN-DASHBOARD-SUMMARY.md)** - Executive summary
- **[spec/architecture/admin-dashboard.md](spec/architecture/admin-dashboard.md)** - Full specification (UI mockups, implementation details)

**Result:** Power users get complete visibility and control over the privacy stack. Casual users can use simple presets. Everyone understands what protections are active and how the system is performing.
