use serde::{Deserialize, Serialize};

// TODO: Regex parser

/// Anchor describing where a note belongs in source code.
///
/// Line anchors pin a note to an exact cursor position. Symbol anchors pin a note
/// to a named code symbol and are re-resolved from current source text when viewed.
///
/// 노트가 소스 코드 어디에 속하는지 설명하는 앵커입니다.
///
/// Line 앵커는 정확한 커서 위치에 노트를 고정하고, Symbol 앵커는 이름 있는
/// 코드 심볼에 노트를 고정하며 조회 시 현재 소스 텍스트에서 다시 해석됩니다.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "type")]
pub enum NoteAnchor {
    Line(LineAnchor),
    Symbol(SymbolAnchor),
}

/// Exact line/column anchor stored in vault JSON.
///
/// vault JSON에 저장되는 정확한 line/column 앵커입니다.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct LineAnchor {
    /// 1-based line number in the source file.
    ///
    /// 소스 파일의 1-based 줄 번호입니다.
    pub line: u32,

    /// 1-based column number in the source file.
    ///
    /// 소스 파일의 1-based 열 번호입니다.
    pub column: u32,
}

/// Symbol-based anchor that survives file edits better than raw line numbers.
///
/// `signature` and `line_hint` help re-locate the symbol after source changes.
///
/// 원시 line 번호보다 파일 수정에 더 잘 견디는 symbol 기반 앵커입니다.
///
/// `signature`와 `line_hint`는 소스 변경 후 심볼을 다시 찾는 데 도움을 줍니다.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SymbolAnchor {
    pub name: String,
    pub kind: SymbolKind,
    pub signature: Option<String>,
    pub line_hint: Option<u32>,
}

/// Symbol category used when matching source markers.
///
/// 소스 마커를 매칭할 때 사용하는 심볼 분류입니다.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum SymbolKind {
    Function,
    Struct,
    Enum,
    Trait,
    Impl,
    Method,
    Unknown,
}
