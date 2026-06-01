# FrilVault

> Your Personal Knowledge Vault for Code

🚧 Early development stage

FrilVault는 개발자를 위한 개인 지식 저장소(Personal Knowledge Vault)입니다.

소스 코드에 직접 주석을 남기지 않고도 코드 분석 내용, 학습 기록, TODO, 아이디어, 디버깅 메모를 프로젝트와 함께 관리할 수 있습니다.

모든 노트는 프로젝트 내부의 `.vault` 폴더에 저장되며, 필요에 따라 `.gitignore`를 통해 개인 전용 데이터로 유지할 수 있습니다.

---

## Why FrilVault?

오픈소스를 분석하거나 대규모 프로젝트를 학습할 때 다음과 같은 문제가 자주 발생합니다.

```rust
pub fn change_physical_size_smooth(...) {
    ...
}
```

직접 주석을 추가하면:

- 원본 코드가 오염됨
- 업스트림 업데이트 시 충돌 발생
- PR 생성 시 불필요한 변경 포함

FrilVault는 코드를 수정하지 않고 별도의 노트 파일에 분석 내용을 저장합니다.

```yaml
notes:
  - symbol: ScreenScaler::change_physical_size_smooth

    tags:
      - bug
      - solved

    comment: |
      smooth_gutter_x 초기화 누락 문제.
      PR #108에서 수정됨.
```

---

## Features

### Personal Notes

코드에 직접 주석을 작성하지 않고 개인 노트를 저장합니다.

### Symbol Based Notes

라인 번호가 아닌 심볼(Function, Struct, Class, Method)에 연결됩니다.

```yaml
symbol: UserService::createUser
```

### Project Knowledge

프로젝트를 분석하며 축적한 지식을 저장합니다.

### Search

심볼, 태그, 키워드 기반 검색을 제공합니다.

### AI Context

저장된 노트를 AI 컨텍스트로 활용할 수 있습니다.

### Local First

모든 데이터는 로컬에 저장됩니다.

---

## Example Structure

```text
project/
├── src/
├── .vault/
│   ├── workspace.yml
│   ├── notes/
│   │   ├── scaler.rs.yml
│   │   ├── shader.rs.yml
│   │   └── user.service.yml
│   └── cache/
└── .gitignore
```

---

## Use Cases

### Open Source Analysis

라이브러리와 프레임워크를 분석하며 학습 기록을 저장합니다.

### Reverse Engineering

코드 흐름과 의존 관계를 기록합니다.

### Personal Documentation

프로젝트 설계 의도와 구현 메모를 관리합니다.

### AI Assisted Development

AI가 프로젝트를 더 잘 이해할 수 있도록 개인 컨텍스트를 제공합니다.

---

## Future Roadmap

- VSCode Extension
- Symbol Indexing
- Project Search
- Semantic Search
- AI Context Engine
- RAG Integration
- JetBrains Plugin
- Desktop Application

---

## Philosophy

Source code should remain clean.

Knowledge belongs in the vault.
