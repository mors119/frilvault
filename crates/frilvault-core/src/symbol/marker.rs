use crate::{SymbolAnchor, SymbolKind};

pub fn symbol_marker(symbol: &SymbolAnchor) -> String {
    if let Some(signature) = &symbol.signature {
        return signature.clone();
    }

    symbol_marker_for_name(&symbol.name, symbol.kind)
}

pub fn symbol_marker_for_name(name: &str, kind: SymbolKind) -> String {
    match kind {
        SymbolKind::Struct => format!("struct {name}"),
        SymbolKind::Enum => format!("enum {name}"),
        SymbolKind::Trait => format!("trait {name}"),
        SymbolKind::Function | SymbolKind::Method => format!("fn {name}"),
        SymbolKind::Impl => format!("impl {name}"),
        SymbolKind::Unknown => name.to_string(),
    }
}
