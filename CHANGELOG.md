# Changelog

All notable changes to MagicAPI AI Gateway will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.6] - 2024-11-13
### Added
- Support for Fireworks AI provider
  - Complete integration with Fireworks API
  - Streaming and non-streaming support
  - Model-specific optimizations
- Support for Together.ai provider
  - Full API integration
  - Support for all Together.ai models
  - Streaming capabilities
### Enhanced
- Documentation updates for new providers
- Example usage for all supported providers
- Performance optimizations for streaming responses

## [0.1.5] - 2024-11-13
### Added
- Anthropic Claude support with automatic path transformation
- Provider framework restructuring
- Unified provider interface with trait-based implementation
### Enhanced
- Provider-specific path transformations
- Header processing across all providers
- Authentication flow standardization
### Fixed
- Header processing for streaming responses
- Error handling for invalid API keys
- Provider-specific status code handling

## [0.1.4] - 2024-11-07
### Added
- Docker support with multi-stage builds
- Docker Compose configuration
### Enhanced
- Documentation improvements
- Build process optimization

## [0.1.3] - 2024-11-07
### Added
- GROQ provider support
- Native integration with GROQ's ultra-fast LLM API
### Enhanced
- Stream handling improvements
- Error message clarity
- Request timeout handling
### Fixed
- Stream handling edge cases
- Memory management for long-running streams

## [0.1.0] - 2024-11-07
### Added
- Initial release
- Basic provider framework
- OpenAI support
- Streaming capabilities
- Error handling
- Basic documentation

[Unreleased]: https://github.com/MagicAPI/ai-gateway/compare/v0.1.6...HEAD
[0.1.6]: https://github.com/MagicAPI/ai-gateway/compare/v0.1.5...v0.1.6
[0.1.5]: https://github.com/MagicAPI/ai-gateway/compare/v0.1.4...v0.1.5
[0.1.3]: https://github.com/MagicAPI/ai-gateway/compare/v0.1.0...v0.1.3