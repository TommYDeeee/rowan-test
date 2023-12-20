use std::{
    borrow::Borrow,
    fmt,
    mem::{self, ManuallyDrop},
    ops, ptr,
};

use countme::Count;

use crate::{
    arc::{Arc, HeaderSlice, ThinArc},
    green::SyntaxKind,
    TextSize,
};

#[derive(PartialEq, Eq, Hash)]
struct GreenTokenHead {
    kind: SyntaxKind,
    _c: Count<GreenToken>,
}

type Repr = HeaderSlice<GreenTokenHead, [u8]>;
type ReprThin = HeaderSlice<GreenTokenHead, [u8; 0]>;
#[repr(transparent)]
pub struct GreenTokenData {
    data: ReprThin,
}

impl PartialEq for GreenTokenData {
    fn eq(&self, other: &Self) -> bool {
        self.kind() == other.kind() && self.text() == other.text()
    }
}

/// Leaf node of the immutable tree
#[derive(Clone, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct GreenToken {
    ptr: ThinArc<GreenTokenHead, u8>,
}

impl ToOwned for GreenTokenData {
    type Owned = GreenToken;

    #[inline]
    fn to_owned(&self) -> GreenToken {
        unsafe {
            let green = ManuallyDrop::new(GreenToken::from_raw(ptr::NonNull::from(self)));
            GreenToken::clone(&green)
        }
    }
}

impl Borrow<GreenTokenData> for GreenToken {
    #[inline]
    fn borrow(&self) -> &GreenTokenData {
        &*self
    }
}

impl fmt::Debug for GreenTokenData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("GreenToken")
            .field("kind", &self.kind())
            .field("text", &self.text())
            .finish()
    }
}

impl fmt::Debug for GreenToken {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let data: &GreenTokenData = &*self;
        fmt::Debug::fmt(data, f)
    }
}

impl fmt::Display for GreenToken {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let data: &GreenTokenData = &*self;
        fmt::Display::fmt(data, f)
    }
}

impl fmt::Display for GreenTokenData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.text())
    }
}

impl GreenTokenData {
    /// Kind of this token
    #[inline]
    pub fn kind(&self) -> SyntaxKind {
        self.data.header.kind
    }

    /// Text of this Token
    #[inline]
    pub fn text(&self) -> &str {
        unsafe { std::str::from_utf8_unchecked(self.data.slice()) }
    }

    /// Length of the text that is covered by this token
    pub fn text_len(&self) -> TextSize {
        TextSize::of(self.text())
    }
}

impl GreenToken {
    /// Creates a new token
    #[inline]
    pub fn new(kind: SyntaxKind, text: &str) -> GreenToken {
        let head = GreenTokenHead {
            kind,
            _c: Count::new(),
        };
        // this is just to alloc(usize(for rc) + size_of(head) + text.len())
        let ptr = ThinArc::from_header_and_iter(head, text.bytes());
        GreenToken { ptr }
    }

    #[inline]
    pub(crate) fn into_raw(this: GreenToken) -> ptr::NonNull<GreenTokenData> {
        let green: &GreenTokenData = &*ManuallyDrop::new(this);
        ptr::NonNull::from(&*green)
    }

    #[inline]
    pub(crate) unsafe fn from_raw(ptr: ptr::NonNull<GreenTokenData>) -> GreenToken {
        let arc = mem::transmute::<Arc<ReprThin>, ThinArc<GreenTokenHead, u8>>(Arc::from_raw(
            &ptr.as_ref().data as *const ReprThin,
        ));
        GreenToken { ptr: arc }
    }
}

impl ops::Deref for GreenToken {
    type Target = GreenTokenData;

    // Lot of unsafe code just to basically code a identity function
    #[inline]
    fn deref(&self) -> &GreenTokenData {
        unsafe {
            let repr: &Repr = &self.ptr;
            let repr: &ReprThin = &*(repr as *const Repr as *const ReprThin);
            mem::transmute::<&ReprThin, &GreenTokenData>(repr)
        }
    }
}
