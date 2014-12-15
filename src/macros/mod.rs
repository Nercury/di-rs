#![macro_escape]

macro_rules! count_exprs {
    () => (0);
    ($head:expr $(, $tail:expr)*) => (1 + count_exprs!($($tail),*));
}

macro_rules! expr_cmp_many(
    ({ $this:expr, $other:expr }) => ($this.cmp($other));
    ({ $this:expr, $other:expr } $(,{ $tail_this:expr, $tail_other:expr })*) => (
        match $this.cmp($other) {
            Ordering::Equal => {
                expr_cmp_many!($({ $tail_this, $tail_other }),+)
            },
            ordering => ordering,
        }
    );
)

macro_rules! expr_partial_cmp_many(
    ({ $this:expr, $other:expr }) => ($this.partial_cmp($other));
    ({ $this:expr, $other:expr } $(,{ $tail_this:expr, $tail_other:expr })*) => (
        match $this.partial_cmp($other) {
            Some(Ordering::Equal) => {
                expr_partial_cmp_many!($({ $tail_this, $tail_other }),+)
            },
            test => test,
        }
    );
)

macro_rules! expr_eq_many(
    ($({ $this:expr, $other:expr }),+) => (
        $($this.eq($other)) && +
    );
)

macro_rules! ord_for(
    ($Struct:ty { $($field:ident),+ }) => (
        impl PartialOrd for $Struct {
            fn partial_cmp(&self, other: &$Struct) -> Option<Ordering> {
                expr_partial_cmp_many!($({ self.$field, &other.$field }),+)
            }
        }
        impl PartialEq for $Struct {
            fn eq(&self, other: &$Struct) -> bool {
                expr_eq_many!($({ self.$field, &other.$field }),+)
            }
        }
        impl Eq for $Struct {}
        impl Ord for $Struct {
            fn cmp(&self, other: &$Struct) -> Ordering {
                expr_cmp_many!($({ self.$field, &other.$field }),+)
            }
        }
    );
)
