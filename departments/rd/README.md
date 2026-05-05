# 🧬 rd — Phòng Tiến Hóa (R&D)

## Chức năng
Phòng Nghiên cứu & Phát triển — tự động quét các nguồn tài nguyên AI miễn phí (HuggingFace, GitHub), đề xuất nâng cấp model, và duy trì sự tiến hóa liên tục của hệ thống iZcore.

## Cấu trúc
```
departments/rd/
├── src/
│   └── lib.rs      # R&D logic (đang xây dựng)
└── Cargo.toml
```

## Nhiệm vụ chính
| Task | Mô tả | Trạng thái |
|---|---|---|
| HF Scanner | Quét HuggingFace tìm model mới phù hợp | 📋 Planned |
| GitHub Radar | Theo dõi các repo AI/Rust nổi bật mới nhất | 📋 Planned |
| Model Benchmark | Đánh giá và so sánh hiệu suất các LLM | 📋 Planned |
| iZcore Updater | Đề xuất nâng cấp DNA Kernel khi có version mới | 📋 Planned |
| Tech Report | Báo cáo xu hướng công nghệ hàng tuần → BA | 📋 Planned |

## Agent phụ trách: `CTO Omega` — Node `mac-m4` (Apple M4 / 32GB RAM)

## Lưu ý quan trọng
- **Node ưu tiên**: `mac-m4` vì cần RAM cao để chạy model locally.
- **Ưu tiên tài nguyên FREE**: Cloudflare AI, HuggingFace Inference API (free tier), Ollama local.
