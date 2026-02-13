# Project Plan Enhancement Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Enhance the existing project-plan.md with comprehensive team structure, sprint-level execution details, UX/UI design process, security validation, deployment strategy, and collaboration workflows.

**Architecture:** This is a documentation enhancement task. We'll add new sections before the existing phase content while preserving all existing technical details. The document will grow from ~400 lines to ~1,200-1,500 lines with clear navigation via table of contents.

**Tech Stack:** Markdown documentation, Git for version control

---

## Task 1: Add Executive Summary and Enhanced Table of Contents

**Files:**
- Modify: `project-plan.md:1-7` (replace version header)

**Step 1: Backup existing version info**

Note the current version, created date, and status for reference.

**Step 2: Write new header with Executive Summary**

Add to beginning of `project-plan.md` (before Phase 0):

```markdown
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
```

**Step 3: Verify formatting**

Ensure markdown renders correctly with anchor links working.

**Step 4: Commit the changes**

```bash
git add project-plan.md
git commit -m "docs: add executive summary and enhanced table of contents

- Add comprehensive executive summary with mission, scope, approach
- Add detailed table of contents with sections for planning, phases, and supporting info
- Prepare structure for comprehensive project plan enhancement
- Update version to 2.0.0

Co-Authored-By: Claude Sonnet 4.5 <noreply@anthropic.com>"
```

---

## Task 2: Add Team Structure & Roles Section

**Files:**
- Modify: `project-plan.md` (after Table of Contents, before Phase 0)

**Step 1: Add Team Structure section header**

Insert after Table of Contents:

```markdown
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
```

**Step 2: Commit Team Structure section**

```bash
git add project-plan.md
git commit -m "docs: add comprehensive team structure and roles

- Add 25-40 person team breakdown with scaling strategy
- Define 4 core leadership roles
- Detail 5 engineering teams (Crypto, Mobile, Backend, Wallet)
- Define Design & UX team (5-6 people) with UX obsession philosophy
- Define Security & QA team with red team approach
- Define DevOps team with deployment promises
- Include phase-by-phase scaling strategy (29 → 38 → 41 → 18)

Co-Authored-By: Claude Sonnet 4.5 <noreply@anthropic.com>"
```

---

## Task 3: Add Development Workflow & Sprint Process Section

**Files:**
- Modify: `project-plan.md` (after Team Structure, before Phase 0)

**Step 1: Add Development Workflow section**

Insert comprehensive sprint workflow section (content from design doc, adapted for project-plan.md format).

**Step 2: Commit Development Workflow section**

```bash
git add project-plan.md
git commit -m "docs: add development workflow and sprint process

- Define 2-week sprint structure (Week 1 and Week 2 cadence)
- Detail design-led development process (design must be validated before engineering)
- Define code quality gates (PR requirements, automated CI/CD)
- Define Definition of Done (7 criteria including design QA and security testing)
- Sprint ceremonies, daily standups, demo process

Co-Authored-By: Claude Sonnet 4.5 <noreply@anthropic.com>"
```

---

## Task 4: Add UX/UI Design-Driven Development Section

**Files:**
- Modify: `project-plan.md` (after Development Workflow, before Phase 0)

**Step 1: Add UX/UI Design-Driven Development section**

Insert comprehensive design process section (5 phases: Discover, Define, Design, Validate, Handoff), design system details, and continuous UX validation.

**Step 2: Commit UX/UI section**

```bash
git add project-plan.md
git commit -m "docs: add UX/UI design-driven development process

- Define design-first philosophy (no code until validated with users)
- Detail 5-phase design process (Discover, Define, Design, Validate, Handoff)
- Design system built in Phase 0-1 (foundation, components, patterns)
- Continuous UX validation (every sprint: design QA, usability testing)
- The UX Obsession: 30-second comprehension rule

Co-Authored-By: Claude Sonnet 4.5 <noreply@anthropic.com>"
```

---

## Task 5: Add Security & QA Continuous Validation Section

**Files:**
- Modify: `project-plan.md` (after UX/UI section, before Phase 0)

**Step 1: Add Security & QA section**

Insert comprehensive security validation process (every sprint, phase-level, continuous QA, red team).

**Step 2: Commit Security & QA section**

```bash
git add project-plan.md
git commit -m "docs: add security and QA continuous validation process

- Security testing in every sprint (code review, threat modeling, pen testing)
- Phase-level security validation (external mini-audits)
- Test coverage requirements (100% crypto/wallet, 85% overall)
- Testing strategy (unit, integration, property, fuzzing, performance, security)
- Red team continuously attacks system
- QA release process with 5-step validation

Co-Authored-By: Claude Sonnet 4.5 <noreply@anthropic.com>"
```

---

## Task 6: Add Deployment & Infrastructure Strategy Section

**Files:**
- Modify: `project-plan.md` (after Security & QA, before Phase 0)

**Step 1: Add Deployment & Infrastructure section**

Insert deployment environments, infrastructure architecture, CI/CD pipeline, monitoring, and easy deployment promises.

**Step 2: Commit Deployment & Infrastructure section**

```bash
git add project-plan.md
git commit -m "docs: add deployment and infrastructure strategy

- Define 5 deployment environments (local, CI/CD, staging, beta, production)
- Mobile app deployment (iOS: Xcode Cloud → TestFlight → App Store)
- Relay node deployment (Terraform + Ansible, one-click Docker setup)
- VPN node deployment (WireGuard, 50+ global locations)
- CI/CD pipeline (on commit, PR, merge, tag)
- Monitoring (Sentry, Prometheus, Grafana, privacy-preserving metrics)
- Easy deployment promises (2 taps for users, 15 min for node operators)

Co-Authored-By: Claude Sonnet 4.5 <noreply@anthropic.com>"
```

---

## Task 7: Add Communication & Collaboration Section

**Files:**
- Modify: `project-plan.md` (after Deployment, before Tech Stack)

**Step 1: Add Communication & Collaboration section**

Insert communication channels, meeting cadence, code review process, knowledge sharing, conflict resolution, remote work, and tools.

**Step 2: Commit Communication & Collaboration section**

```bash
git add project-plan.md
git commit -m "docs: add communication and collaboration processes

- Define communication channels (sync and async)
- Meeting cadence (sprint ceremonies, design reviews, security reviews)
- No-meeting deep work blocks (Tuesdays & Thursdays 10am-2pm)
- Code review process (PR size, review requirements, SLA, etiquette)
- Knowledge sharing (weekly knowledge shares, pair programming, docs culture)
- Conflict resolution process
- Remote/hybrid work policies
- Tools and platforms

Co-Authored-By: Claude Sonnet 4.5 <noreply@anthropic.com>"
```

---

## Task 8: Add Enhanced Technology Stack & Tools Section

**Files:**
- Modify: `project-plan.md` (after Communication, before Phase 0)

**Step 1: Add enhanced Technology Stack section**

Insert expanded technology stack with justifications (existing content from CLAUDE.md enhanced).

**Step 2: Commit Technology Stack enhancement**

```bash
git add project-plan.md
git commit -m "docs: add enhanced technology stack and tools section

- Expand existing tech stack with detailed justifications
- Add development tools (Git, CI/CD, IDE)
- Add design tools (Figma, prototyping, user testing)
- Add communication tools (Slack/Discord, Zoom, Loom)
- Add project management tools (Jira/Linear, Notion)
- Add monitoring tools (Sentry, Grafana, PagerDuty)

Co-Authored-By: Claude Sonnet 4.5 <noreply@anthropic.com>"
```

---

## Task 9: Enhance Phase 0 with Team Composition and Sprint Breakdown

**Files:**
- Modify: `project-plan.md:9-22` (Phase 0 section)

**Step 1: Add Team Composition for Phase 0**

Insert after Phase 0 milestone table:

```markdown
### Team Composition - Phase 0

**Active Teams:**
- **Crypto & Core Team:** 5 engineers (100% allocation)
- **Mobile Team:** 4 engineers (40% allocation - project setup, architecture)
- **Backend Team:** 3 engineers (30% allocation - relay node architecture planning)
- **Design Team:** 4 designers (25% allocation - design system foundation, onboarding flows)
- **Security Team:** 2 engineers (50% allocation - threat modeling, crypto review)
- **DevOps Team:** 3 engineers (75% allocation - CI/CD setup, infrastructure planning)

**Total: 21 people active in Phase 0**

---

### Sprint Breakdown - Phase 0 (4 Sprints)

#### Sprint 0.1 (Weeks 1-2): Project Setup + Crypto Core Foundation

**Sprint Goal:** Setup development infrastructure and begin Signal Protocol implementation

**Design Work (Pre-Sprint, Weeks -2 to 0):**
- Design system color palette and typography (Design Team)
- Initial onboarding flow wireframes (UX Researcher + Product Designer)
- App architecture and navigation patterns (Product Designer)

**Engineering Tasks:**
- **Crypto Team:**
  - [ ] Setup Rust workspace structure (Crypto Lead)
  - [ ] Implement X3DH key agreement with NIST test vectors (2x Crypto Engineers)
  - [ ] Setup memory-locked regions for crypto operations (Security Engineer)
  - [ ] Property testing infrastructure for X3DH (Test Engineer)
- **Mobile Team:**
  - [ ] Flutter project scaffolding (iOS + Android) (2x Flutter Engineers)
  - [ ] Platform channel setup for native crypto (iOS Specialist)
- **Backend Team:**
  - [ ] Relay node project structure (Rust Engineer)
  - [ ] Protocol buffer schemas for message format (Network Engineer)
- **DevOps Team:**
  - [ ] GitHub Actions CI/CD pipeline (DevOps Lead)
  - [ ] Docker Compose local dev environment (Infrastructure Engineer)
  - [ ] Reproducible build setup (Release Engineer)

**Security Focus:**
- Threat model for key storage and memory protection
- Review crypto library choices (libsignal vs ring vs native)
- Setup fuzzing infrastructure (AFL for X3DH implementation)

**Success Criteria:**
- ✓ X3DH key agreement passes all NIST test vectors
- ✓ CI pipeline runs tests on every commit
- ✓ Local dev environment setup with `./scripts/dev-setup.sh`
- ✓ Security threat model document created and reviewed
- ✓ Design system foundation (colors, typography) approved by Design Lead

**Risks & Mitigations:**
- **Risk:** ML-KEM library immaturity → **Mitigation:** Have X25519-only fallback ready
- **Risk:** Team still onboarding, slower velocity → **Mitigation:** Pair programming required for all crypto code
- **Risk:** Design system decisions blocked → **Mitigation:** Design Lead has final call, document decision rationale

**Sprint Demo:**
- Working X3DH key exchange demonstration (Crypto Team)
- Flutter app loads on iOS and Android simulators (Mobile Team)
- CI pipeline demonstration (DevOps Team)
- Design system foundations presentation (Design Team)

---

#### Sprint 0.2 (Weeks 3-4): Double Ratchet + Post-Quantum Key Exchange

**Sprint Goal:** Complete Signal Protocol Double Ratchet and add post-quantum protection

**Design Work (Pre-Sprint, Weeks 1-2):**
- Onboarding flow high-fidelity prototypes (Product Designer)
- First-time key generation UX (Product Designer + UX Researcher)
- User testing of onboarding flow (8 sessions, UX Researcher)

**Engineering Tasks:**
- **Crypto Team:**
  - [ ] Double Ratchet sending chain implementation (Crypto Engineer 1)
  - [ ] Double Ratchet receiving chain implementation (Crypto Engineer 2)
  - [ ] PQXDH hybrid key exchange (ML-KEM-1024 + X25519) (Security Engineer)
  - [ ] Integration tests: X3DH + Double Ratchet (Test Engineer)
  - [ ] Constant-time operations audit (Performance Engineer)
- **Mobile Team:**
  - [ ] FFI bindings for Rust crypto lib (Flutter Engineer 1)
  - [ ] Key generation UI implementation (Flutter Engineer 2)
  - [ ] Biometric authentication integration (iOS Specialist)
- **Backend Team:**
  - [ ] Message routing protocol design (Network Engineer)
  - [ ] Dead drop mailbox data structures (Rust Engineer)
- **DevOps Team:**
  - [ ] Security scanning in CI (cargo-audit, dependency checks) (DevOps Lead)

**Security Focus:**
- Formal verification of Double Ratchet invariants (with external tool)
- Side-channel analysis of key derivation (timing attacks)
- Memory safety audit (all sensitive data zeroized)

**Success Criteria:**
- ✓ Double Ratchet encrypts and decrypts messages correctly
- ✓ PQXDH key exchange integrates with X3DH
- ✓ Post-quantum shared secret properly derived via HKDF
- ✓ All crypto operations constant-time (verified with valgrind)
- ✓ Biometric authentication works on iOS simulator
- ✓ User testing shows 80%+ comprehension of onboarding flow

**Risks & Mitigations:**
- **Risk:** PQXDH integration complexity → **Mitigation:** Rust ML-KEM crate already vetted, clear spec from NIST
- **Risk:** FFI overhead impacts performance → **Mitigation:** Benchmark all FFI calls, optimize if >10ms
- **Risk:** User testing reveals confusion → **Mitigation:** Design team iterates, re-tests before Sprint 0.3

**Sprint Demo:**
- End-to-end encrypted message (X3DH + PQXDH + Double Ratchet) (Crypto Team)
- Key generation with biometric UI (Mobile Team)
- Design system component library v0.1 (Design Team)

---

#### Sprint 0.3 (Weeks 5-6): Encrypted Storage + Device Identity

[Similar structure for Sprint 0.3...]

#### Sprint 0.4 (Weeks 7-8): Daily Key Rotation + Phase 0 Completion

[Similar structure for Sprint 0.4...]

---
```

**Step 2: Commit Phase 0 enhancement**

```bash
git add project-plan.md
git commit -m "docs: enhance Phase 0 with team composition and sprint breakdown

- Add team composition: 21 people active in Phase 0
- Add Sprint 0.1 (Weeks 1-2): Project setup + crypto core foundation
- Add Sprint 0.2 (Weeks 3-4): Double Ratchet + post-quantum key exchange
- Each sprint includes: goal, design work, engineering tasks, security focus, success criteria, risks, demo
- Sprint structure template established for remaining phases

Co-Authored-By: Claude Sonnet 4.5 <noreply@anthropic.com>"
```

---

## Task 10: Add Success Metrics & Monitoring Section

**Files:**
- Modify: `project-plan.md` (after all phases, before existing appendices)

**Step 1: Add Success Metrics section**

Insert comprehensive success metrics section (engineering, design, security, deployment metrics).

**Step 2: Commit Success Metrics section**

```bash
git add project-plan.md
git commit -m "docs: add success metrics and monitoring section

- Engineering metrics (milestone delivery, test coverage, performance)
- Design metrics (user testing success rate, accessibility, usability)
- Security metrics (audit findings, vulnerabilities, threat model)
- Deployment metrics (staging success, dogfooding, monitoring setup)
- Network health metrics (relay uptime, latency, community nodes)

Co-Authored-By: Claude Sonnet 4.5 <noreply@anthropic.com>"
```

---

## Task 11: Add Risk Management & Mitigation Section

**Files:**
- Modify: `project-plan.md` (after Success Metrics, before existing appendices)

**Step 1: Add Risk Management section**

Insert comprehensive risk management section (technical risks, team risks, security risks, deployment risks).

**Step 2: Commit Risk Management section**

```bash
git add project-plan.md
git commit -m "docs: add risk management and mitigation section

- Technical risks (crypto complexity, performance, platform compatibility)
- Team risks (scaling challenges, knowledge silos, burnout)
- Security risks (undiscovered vulnerabilities, side-channel attacks)
- Deployment risks (app store rejections, node operator adoption)
- Mitigation strategies for each risk category

Co-Authored-By: Claude Sonnet 4.5 <noreply@anthropic.com>"
```

---

## Task 12: Final Review and Formatting

**Files:**
- Modify: `project-plan.md` (entire document)

**Step 1: Review entire document for consistency**

Check:
- All anchor links work correctly
- Formatting is consistent throughout
- No duplicate content
- All sections flow logically
- Markdown renders correctly

**Step 2: Add note about remaining phases**

After Sprint 0.2 section, add:

```markdown
---

**Note:** Sprints 0.3 and 0.4 follow similar structure. Full sprint breakdowns for Phase 1-7 will be added incrementally as each phase approaches, following the same template: Sprint Goal, Design Work, Engineering Tasks, Security Focus, Success Criteria, Risks & Mitigations, and Sprint Demo.

The sprint structure demonstrated in Phase 0 serves as the template for all subsequent phases. Each phase will be broken into 2-week sprints with clear team assignments, deliverables, and validation gates.

---
```

**Step 3: Update version and date**

Verify version is 2.0.0 and updated date is 2026-02-13.

**Step 4: Final commit**

```bash
git add project-plan.md
git commit -m "docs: final review and formatting of comprehensive project plan

- Verify all anchor links work
- Add note about incremental sprint breakdown for Phase 1-7
- Confirm version 2.0.0 and updated date
- Document ready for team review and use
- Enhanced from ~400 to ~1,200 lines with complete team structure, processes, and execution details

Co-Authored-By: Claude Sonnet 4.5 <noreply@anthropic.com>"
```

---

## Implementation Notes

**Document Size:** The completed project-plan.md will be approximately 1,200-1,500 lines (manageable with table of contents navigation).

**Remaining Work:** After this plan is executed:
- Full sprint breakdowns for Phase 1-7 can be added incrementally as each phase approaches
- The structure established in Phase 0 (Sprints 0.1-0.2) serves as the template
- Each subsequent phase follows the same pattern: Team Composition + Sprint Breakdown

**Usage:** The enhanced project-plan.md becomes the single source of truth for:
- New team member onboarding
- Sprint planning reference
- Cross-team coordination
- Stakeholder communication
- Development process documentation

---

## Completion Checklist

- [ ] Task 1: Executive Summary and TOC
- [ ] Task 2: Team Structure & Roles
- [ ] Task 3: Development Workflow
- [ ] Task 4: UX/UI Design-Driven Development
- [ ] Task 5: Security & QA Validation
- [ ] Task 6: Deployment & Infrastructure
- [ ] Task 7: Communication & Collaboration
- [ ] Task 8: Technology Stack & Tools
- [ ] Task 9: Phase 0 Enhancement (team + sprints)
- [ ] Task 10: Success Metrics & Monitoring
- [ ] Task 11: Risk Management & Mitigation
- [ ] Task 12: Final Review and Formatting

**When complete:** project-plan.md is transformed from a phase-milestone plan into a comprehensive build-and-deploy operations manual for a 25-40 person team executing an 88-week project.
