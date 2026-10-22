# elendirna (npm)

AI-friendly knowledge vault for carrying the history behind your ideas across models and services. The npm package distributes the `elf` CLI and MCP server as prebuilt binaries; a Rust toolchain is not required.

아이디어가 형성된 맥락과 변화 이력을 모델·서비스 사이에서 이어 주는 지식 vault입니다. npm 패키지는 `elf` CLI와 MCP 서버를 사전 빌드 바이너리로 제공하므로 Rust 도구 체인이 필요하지 않습니다.

For the project overview, concepts, and Cargo installation, see the [main README](../../README.md).
프로젝트 개요와 개념, Cargo 설치 방법은 [루트 README](../../README.md)를 참고하세요.

## Install / 설치

```sh
# one-off / 일회 실행
npx -y elendirna --help

# global install / 전역 설치
npm install -g elendirna
elendirna --help
```

The package bundles binaries for Linux, macOS, and Windows on x64 and arm64. It has no `postinstall` script and requires no network access at install or run time. Linux binaries require glibc 2.35 or newer; use `cargo install eln-cli` on musl/Alpine.

Linux, macOS, Windows의 x64 및 arm64 바이너리를 포함합니다. `postinstall` 스크립트가 없고 설치·실행 시 네트워크를 요구하지 않습니다. Linux 바이너리는 glibc 2.35 이상이 필요하며, musl/Alpine에서는 `cargo install eln-cli`를 사용하세요.

## MCP server / MCP 서버

After global installation, run `elendirna serve --snippet` to print an MCP client configuration snippet. To run the stdio server directly, configure the client with:
전역 설치 후 `elendirna serve --snippet`을 실행하면 MCP 클라이언트 설정 예시가 출력됩니다. stdio 서버를 직접 실행하려면 다음과 같이 설정합니다.

```json
{
  "mcpServers": {
    "elendirna": {
      "command": "elendirna",
      "args": ["serve", "--mcp", "--transport", "stdio", "--vault", "/path/to/vault"]
    }
  }
}
```

For an npx-only setup, use `command: "npx"` and `args: ["-y", "elendirna", "serve", "--mcp", "--transport", "stdio", "--vault", "/path/to/vault"]`. Replace `/path/to/vault` with the vault you intend to share.

npx만 사용하는 경우 `command`를 `npx`로, `args`를 `["-y", "elendirna", "serve", "--mcp", "--transport", "stdio", "--vault", "/path/to/vault"]`로 지정하세요. `/path/to/vault`는 AI 클라이언트와 공유할 vault 경로로 바꾸세요.

## Source / 소스

Rust source, issues, and documentation: <https://github.com/elen-labs/elendirna>.

Rust 소스 코드, 이슈, 문서: <https://github.com/elen-labs/elendirna>.
