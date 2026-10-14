# Ogma (`ogma-rs`) — Master System Architecture, 30-D Manifold & DocDD/TDD Specification

## 1. Executive Synthesis & Problem Statement

Large Language Models trained via Autoregressive Maximum Likelihood Estimation and aligned with Reinforcement Learning from Human Feedback (RLHF) or Direct Preference Optimization (DPO) suffer from **structural mode collapse**. While first-generation AI detectors targeted transient surface vocabulary—such as the excess frequency of words like *"delve,"* *"tapestry,"* *"underscore,"* and *"pivotal"* documented in [Kobak et al. (2025)](https://www.researchgate.net/publication/393334463_Delving_into_LLM-assisted_writing_in_biomedical_publications_through_excess_vocabulary)—landmark studies in computational narratology and discourse structure prove that the fundamental signature of machine prose is **geometric and macro-structural**.

Across both long-form creative fiction in [StoryScope: Investigating idiosyncrasies in AI fiction (Russell et al., 2026)](https://arxiv.org/html/2604.03136v6) and commercial/expository web writing in [SlopShape: Identifying AI-Generated Commercial Web Content (Madler, 2026)](https://arxiv.org/html/2609.15369v3), researchers demonstrated that when documents are mapped into a high-dimensional vector space of structural decisions, all major frontier model families (**OpenAI GPT**, **Anthropic Claude**, **Google Gemini**, **DeepSeek**, and **Moonshot Kimi**) collapse into a tight, symmetrical, low-variance cylinder near the statistical centroid. Human authors, by contrast, inhabit a dispersed, heavy-tailed manifold defined by structural asymmetry, non-linear temporal ordering, external cultural anchoring, and high nearest-neighbor Euclidean distance.

Crucially, both studies established that **surface-level paraphrasing cannot erase this structural shape**. When AI-generated drafts are rewritten using professional span-level editing pipelines such as [LAMP (Chakrabarty et al., 2025)](https://arxiv.org/html/2409.14509v4) to eliminate purple prose, clichés, and redundant exposition, structural classifiers still identify AI fiction at **93.9% macro-F1** (a drop of only 1.6 points from 95.5% in [StoryScope §4.2](https://arxiv.org/html/2604.03136v6)) and AI commercial writing at **96.1% macro-F1** (a drop of only 0.9 points from 97.0% in [SlopShape §5.4](https://arxiv.org/html/2609.15369v3)).

Named after [Ogma](https://en.wikipedia.org/wiki/Ogma) (the Celtic deity of eloquence, rhetoric, and inventor of the structural notch-and-stem Ogham script), **`Ogma` (`ogma-rs`)** (`src/lib.rs` + `ogma_sdk.py`) is a deterministic, sub-millisecond Rust analysis and course-correction engine built under strict **Documentation-Driven Development (DocDD)** and **Test-Driven Development (TDD)** governance with **100.00% verified test code coverage across all 6 test tiers (`56/56` tests: Unit, Property/Invariant, Integration, BDD, Benchmark, and End-to-End `E2E`)**.

## 2. Mathematical Formulation of the 30-D Manifold & Dual Gate

Let $\mathbf{x} \in [0, 1]^{30}$ be the raw 30-dimensional structural feature vector extracted from a document, and let $\mathbf{z} \in \mathbb{R}^{30}$ be its standardized z-score vector relative to the $N = 600$ reference manifold ($N_H = 100$ human documents, $N_{AI} = 500$ documents across GPT, Claude, Gemini, DeepSeek, and Kimi):

1. **Z-Score Standardization**:
   $$z_j = \frac{x_j - \mu_j}{\sqrt{\sigma_j^2 + 10^{-8}}}, \quad j \in \{0, \dots, 29\}$$
2. **25-Nearest-Neighbor ($k=25$) Euclidean Rarity Score ($R_{25}$)**:
   $$R_{25}(\mathbf{z}) = \frac{1}{25} \sum_{m=1}^{25} \left\| \mathbf{z} - \mathbf{z}_{(m)} \right\|_2, \quad P_{\text{rarity}}(\mathbf{z}) = \frac{1}{N} \sum_{i=1}^{N} \mathbb{I}\left( R_{25}(\mathbf{z}^{(i)}) \le R_{25}(\mathbf{z}) \right)$$
3. **Signed Human Linear Discriminant Projection ($S_{\text{LDA}}$)**:
   $$S_{\text{LDA}}(\mathbf{z}) = \left( 1 + \exp\left[ -0.65 \cdot \left(\mathbf{z} - \tfrac{1}{2}(\boldsymbol{\mu}_H + \boldsymbol{\mu}_{AI})\right)^\top \hat{\mathbf{w}}_{\text{disc}} \right] \right)^{-1}$$
   A document passes the **Dual Human-Shape Gate** when $P_{\text{rarity}}(\mathbf{z}) \ge 0.70$ and $S_{\text{LDA}}(\mathbf{z}) \ge 0.60$.

## 3. The 30 Unified Manifold Dimensions (`00`–`29`)

* **StoryScope Narrative Dimensions (`00`–`19`)**: Grounded in [NarraBench (Hamilton et al., 2025)](https://arxiv.org/html/2510.09869v1) and [StoryScope (Russell et al., 2026)](https://arxiv.org/html/2604.03136v6) (`NAR_SOM_001`..`NAR_END_020`).
* **SlopShape Expository & Universal Syntactic Dimensions (`20`–`29`)**: Grounded in [SlopShape (Madler, 2026)](https://arxiv.org/html/2609.15369v3) and [9 Layers of AI Slop (Ouyang, 2026)](https://buildtolaunch.ai/p/make-ai-writing-sound-human) (`STR_STG_008`, `STR_STG_001`, `VOC_PRT_005`, `STR_FLW_005`, `STR_FLW_006`, `TIM_CUE_011`, `VOC_CRD_001`, `TIM_CUE_003`, `SYN_CNT_001`, `SYN_BRS_002`).
