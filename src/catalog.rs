//! Elytra sağlayıcı kataloğu — GERÇEK sağlayıcılar ve GERÇEK model kimlikleri.
//!
//! ## Kurallar (proje kararı)
//! - **Yerel/Ollama sağlayıcısı YOKTUR** — yalnız bulut sağlayıcıları.
//! - Uydurma model kimliği YOKTUR. Aşağıdaki kimlikler doğrulanmış kaynaklardan:
//!   * OpenRouter ücretsiz modelleri → `GET https://openrouter.ai/api/v1/models`
//!     (16.10.2026 tarihli canlı liste)
//!   * Nous Portal modelleri → kullanıcının kendi hesabında çalıştığı doğrulanmış
//!     kimlikler
//!   * Büyük sağlayıcıların modelleri → Ekim 2026 kamu benchmark/model kartları
//!
//! ## Tazelik
//! Model listeleri başlangıç setidir. İstemci katmanı her sağlayıcının
//! `/models` ucundan CANLI listeyi çekip bununla birleştirebilir
//! (`Provider::models_url()`).

use crate::model::Model;
use crate::provider::{ApiKind, Provider};

// ─────────────────────────── 1. AGREGATÖRLER ───────────────────────────

/// OpenRouter — tek anahtarla yüzlerce modele erişim (ücretsiz modeller dahil).
pub const OPENROUTER: Provider = Provider {
    id: "openrouter",
    name: "OpenRouter",
    hint: "Tek anahtarla 460+ model · 15 ücretsiz model",
    base_url: "https://openrouter.ai/api/v1",
    env_key: "OPENROUTER_API_KEY",
    kind: ApiKind::OpenAiCompatible,
    is_free_tier: true,
    signup: "https://openrouter.ai/keys",
    models: &[
        Model::new("thinkingmachines/inkling:free", "Inkling (Thinking Machines)", 1_048_576, true).coding(),
        Model::new("thinkingmachines/inkling-small:free", "Inkling Small", 1_048_576, true).coding(),
        Model::new("nvidia/nemotron-3.5-super-vl:free", "Nemotron 3.5 Super VL", 1_000_000, true),
        Model::new("nvidia/nemotron-3.5-lightning:free", "Nemotron 3.5 Lightning", 1_000_000, true),
        Model::new("nvidia/nemotron-3-ultra-550b-a55b:free", "Nemotron 3 Ultra 550B", 1_000_000, true).reasoning(),
        Model::new("nvidia/nemotron-3-super-120b-a12b:free", "Nemotron 3 Super 120B", 262_144, true),
        Model::new("poolside/laguna-s-2.1:free", "Laguna S 2.1", 262_144, true).coding(),
        Model::new("poolside/laguna-xs-2.1:free", "Laguna XS 2.1", 262_144, true).coding(),
        Model::new("cohere/north-mini-code:free", "North Mini Code", 256_000, true).coding(),
        Model::new("google/gemma-4-31b-it:free", "Gemma 4 31B", 262_144, true),
        Model::new("google/gemma-4-26b-a4b-it:free", "Gemma 4 26B A4B", 262_144, true),
        Model::new("dots-studio/dots-3-note-preview:free", "Dots3-Note Preview", 512_000, true),
        Model::new("liquid/lfm-2.5-2.6b:free", "LFM2.5 2.6B (küçük/hızlı)", 65_536, true),
    ],
};

// ─────────────────────── 2. DOĞRUDAN SAĞLAYICILAR ───────────────────────

/// Nous Research — Gülmezcetiner projesinin ölçümlerinde kullanılan portal.
pub const NOUS: Provider = Provider {
    id: "nous",
    name: "Nous Research",
    hint: "Nous Portal · Hermes + üçüncü taraf modeller (ücretsiz katman var)",
    base_url: "https://inference-api.nousresearch.com/v1",
    env_key: "NOUS_API_KEY",
    kind: ApiKind::OpenAiCompatible,
    is_free_tier: true,
    signup: "https://portal.nousresearch.com",
    models: &[
        Model::new("meituan/longcat-2.5-preview:free", "LongCat 2.5 Preview (ücretsiz)", 128_000, true).coding(),
        Model::new("poolside/laguna-s-2.1:free", "Laguna S 2.1 (ücretsiz)", 262_144, true).coding(),
        Model::new("poolside/laguna-xs-2.1:free", "Laguna XS 2.1 (ücretsiz)", 262_144, true).coding(),
        Model::new("inclusionai/ling-3.0-flash-sante:free", "Ling 3.0 Flash Sante (ücretsiz)", 131_072, true),
        Model::new("inclusionai/ling-3.0-flash-fin:free", "Ling 3.0 Flash Fin (ücretsiz)", 131_072, true),
        Model::new("stealth/space-bunny-alpha", "Space Bunny Alpha", 128_000, true).reasoning(),
    ],
};

/// Groq — çok hızlı çıkarım (LPU donanımı), cömert ücretsiz katman.
pub const GROQ: Provider = Provider {
    id: "groq",
    name: "Groq Cloud",
    hint: "Ultra hızlı çıkarım · ücretsiz katman",
    base_url: "https://api.groq.com/openai/v1",
    env_key: "GROQ_API_KEY",
    kind: ApiKind::OpenAiCompatible,
    is_free_tier: true,
    signup: "https://console.groq.com/keys",
    models: &[
        Model::new("llama-3.3-70b-versatile", "Llama 3.3 70B Versatile", 128_000, true),
        Model::new("deepseek-r1-distill-llama-70b", "DeepSeek R1 Distill 70B", 128_000, true).reasoning(),
    ],
};

/// Google AI Studio — Gemini ailesi, OpenAI uyumlu köprü.
pub const GOOGLE: Provider = Provider {
    id: "google",
    name: "Google AI Studio",
    hint: "Gemini ailesi · ücretsiz katman",
    base_url: "https://generativelanguage.googleapis.com/v1beta/openai",
    env_key: "GEMINI_API_KEY",
    kind: ApiKind::OpenAiCompatible,
    is_free_tier: true,
    signup: "https://aistudio.google.com/apikey",
    models: &[
        Model::new("gemini-3.5-flash", "Gemini 3.5 Flash", 1_000_000, true),
        Model::new("gemini-3.5-flash-lite", "Gemini 3.5 Flash Lite", 1_000_000, true),
        Model::new("gemini-3.8-flash", "Gemini 3.8 Flash", 1_000_000, true),
    ],
};

/// Cerebras — wafer-scale donanım, çok yüksek token hızı.
pub const CEREBRAS: Provider = Provider {
    id: "cerebras",
    name: "Cerebras",
    hint: "Saniyede binlerce token · ücretsiz katman",
    base_url: "https://api.cerebras.ai/v1",
    env_key: "CEREBRAS_API_KEY",
    kind: ApiKind::OpenAiCompatible,
    is_free_tier: true,
    signup: "https://cloud.cerebras.ai",
    models: &[
        Model::new("llama3.1-8b", "Llama 3.1 8B", 32_768, true),
        Model::new("llama-3.3-70b", "Llama 3.3 70B", 128_000, true),
    ],
};

/// DeepSeek — güçlü kod/akıl yürütme modelleri, düşük fiyat.
pub const DEEPSEEK: Provider = Provider {
    id: "deepseek",
    name: "DeepSeek",
    hint: "Kod ve akıl yürütmede güçlü · düşük fiyat",
    base_url: "https://api.deepseek.com/v1",
    env_key: "DEEPSEEK_API_KEY",
    kind: ApiKind::OpenAiCompatible,
    is_free_tier: false,
    signup: "https://platform.deepseek.com/api_keys",
    models: &[
        Model::new("deepseek-chat", "DeepSeek Chat", 128_000, false),
        Model::new("deepseek-reasoner", "DeepSeek Reasoner", 128_000, false).reasoning(),
    ],
};

/// Mistral — Avrupa merkezli, Codestral kod modeli.
pub const MISTRAL: Provider = Provider {
    id: "mistral",
    name: "Mistral AI",
    hint: "Codestral kod modeli · deneme katmanı",
    base_url: "https://api.mistral.ai/v1",
    env_key: "MISTRAL_API_KEY",
    kind: ApiKind::OpenAiCompatible,
    is_free_tier: true,
    signup: "https://console.mistral.ai/api-keys",
    models: &[
        Model::new("codestral-latest", "Codestral (kod)", 256_000, false).coding(),
        Model::new("mistral-large-latest", "Mistral Large", 128_000, false),
    ],
};

/// Z.ai — GLM ailesi.
pub const ZAI: Provider = Provider {
    id: "zai",
    name: "Z.ai (GLM)",
    hint: "GLM ailesi · açık ağırlıklı sürümleri de var",
    base_url: "https://api.z.ai/api/paas/v4",
    env_key: "ZAI_API_KEY",
    kind: ApiKind::OpenAiCompatible,
    is_free_tier: false,
    signup: "https://z.ai/manage-apikey/apikey-list",
    models: &[
        Model::new("glm-5.3", "GLM 5.3", 1_000_000, false).coding(),
        Model::new("glm-5.3-flash", "GLM 5.3 Flash", 1_000_000, false),
    ],
};

/// Moonshot AI — Kimi ailesi (uzun bağlam).
pub const MOONSHOT: Provider = Provider {
    id: "moonshot",
    name: "Moonshot AI",
    hint: "Kimi ailesi · uzun bağlam",
    base_url: "https://api.moonshot.ai/v1",
    env_key: "MOONSHOT_API_KEY",
    kind: ApiKind::OpenAiCompatible,
    is_free_tier: false,
    signup: "https://platform.moonshot.ai/console/api-keys",
    models: &[
        Model::new("kimi-k3", "Kimi K3", 1_000_000, false).coding(),
    ],
};

/// xAI — Grok ailesi.
pub const XAI: Provider = Provider {
    id: "xai",
    name: "xAI",
    hint: "Grok ailesi",
    base_url: "https://api.x.ai/v1",
    env_key: "XAI_API_KEY",
    kind: ApiKind::OpenAiCompatible,
    is_free_tier: false,
    signup: "https://console.x.ai",
    models: &[
        Model::new("grok-4.6", "Grok 4.6", 500_000, false),
        Model::new("grok-4.5", "Grok 4.5", 500_000, false),
    ],
};

/// OpenAI — GPT ailesi.
pub const OPENAI: Provider = Provider {
    id: "openai",
    name: "OpenAI",
    hint: "GPT ailesi",
    base_url: "https://api.openai.com/v1",
    env_key: "OPENAI_API_KEY",
    kind: ApiKind::OpenAiCompatible,
    is_free_tier: false,
    signup: "https://platform.openai.com/api-keys",
    models: &[
        Model::new("gpt-5.6-sol", "GPT-5.6 Sol", 1_100_000, false).reasoning(),
        Model::new("gpt-5.6-terra", "GPT-5.6 Terra", 1_100_000, false),
        Model::new("gpt-5.6-luna", "GPT-5.6 Luna (hızlı/ucuz)", 1_100_000, false),
    ],
};

/// Anthropic — Claude ailesi (kendi Mesajlar API'si).
pub const ANTHROPIC: Provider = Provider {
    id: "anthropic",
    name: "Anthropic",
    hint: "Claude ailesi · kendi API biçimi",
    base_url: "https://api.anthropic.com/v1",
    env_key: "ANTHROPIC_API_KEY",
    kind: ApiKind::Anthropic,
    is_free_tier: false,
    signup: "https://console.anthropic.com/settings/keys",
    models: &[
        Model::new("claude-opus-5", "Claude Opus 5", 1_000_000, false).reasoning(),
        Model::new("claude-sonnet-5", "Claude Sonnet 5", 1_000_000, false).coding(),
        Model::new("claude-haiku-5.5", "Claude Haiku 5.5 (hızlı)", 1_000_000, false),
    ],
};

/// Together AI — açık ağırlıklı model barındırma.
pub const TOGETHER: Provider = Provider {
    id: "together",
    name: "Together AI",
    hint: "Açık ağırlıklı modeller",
    base_url: "https://api.together.xyz/v1",
    env_key: "TOGETHER_API_KEY",
    kind: ApiKind::OpenAiCompatible,
    is_free_tier: false,
    signup: "https://api.together.ai/settings/api-keys",
    models: &[
        Model::new("meta-llama/Llama-3.3-70B-Instruct-Turbo", "Llama 3.3 70B Turbo", 128_000, false),
    ],
};

/// Fireworks AI — hızlı açık model barındırma.
pub const FIREWORKS: Provider = Provider {
    id: "fireworks",
    name: "Fireworks AI",
    hint: "Hızlı açık model çıkarımı",
    base_url: "https://api.fireworks.ai/inference/v1",
    env_key: "FIREWORKS_API_KEY",
    kind: ApiKind::OpenAiCompatible,
    is_free_tier: false,
    signup: "https://fireworks.ai/account/api-keys",
    models: &[
        Model::new("accounts/fireworks/models/llama-v3p3-70b-instruct", "Llama 3.3 70B", 131_072, false),
    ],
};

/// NVIDIA NIM — NVIDIA'nın barındırdığı açık modeller.
pub const NVIDIA: Provider = Provider {
    id: "nvidia",
    name: "NVIDIA NIM",
    hint: "Açık modeller · deneme kredisi",
    base_url: "https://integrate.api.nvidia.com/v1",
    env_key: "NVIDIA_API_KEY",
    kind: ApiKind::OpenAiCompatible,
    is_free_tier: true,
    signup: "https://build.nvidia.com",
    models: &[
        Model::new("meta/llama-3.3-70b-instruct", "Llama 3.3 70B Instruct", 128_000, true),
    ],
};

// ─────────────────────────── 3. KATALOG ───────────────────────────

/// Tüm sağlayıcılar (15) — sıralama: önce ücretsiz katmanı olanlar.
pub const PROVIDERS: &[Provider] = &[
    // Ücretsiz katmanı olanlar
    OPENROUTER,
    NOUS,
    GROQ,
    GOOGLE,
    CEREBRAS,
    MISTRAL,
    NVIDIA,
    // Ücretli / kredili
    DEEPSEEK,
    ZAI,
    MOONSHOT,
    XAI,
    OPENAI,
    ANTHROPIC,
    TOGETHER,
    FIREWORKS,
];
