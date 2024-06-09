use either::Either::{self, Left, Right};
use rayon::prelude::*;

pub enum IntoIterKind {
    Sequential,
    Parallel,
}

pub trait IntoIteratorWith<K> {
    type Item;
    type IntoIter;

    fn into_iter_with(self, kind: K) -> Self::IntoIter;
}

impl<T> IntoIteratorWith<IntoIterKind> for T
where
    T: IntoIterator + IntoParallelIterator,
{
    type Item = <T as IntoIterator>::Item;
    type IntoIter = Either<<T as IntoIterator>::IntoIter, <T as IntoParallelIterator>::Iter>;

    fn into_iter_with(self, kind: IntoIterKind) -> Self::IntoIter {
        match kind {
            IntoIterKind::Sequential => Left(self.into_iter()),
            IntoIterKind::Parallel => Right(self.into_par_iter()),
        }
    }
}

pub trait IntoRefIteratorWith<'data, K> {
    type Item: 'data;
    type IntoIter;

    fn iter_with(&'data self, kind: K) -> Self::IntoIter;
}

impl<'data, T: 'data> IntoRefIteratorWith<'data, IntoIterKind> for T
where
    &'data T: IntoIterator + IntoParallelIterator,
{
    type Item = <&'data T as IntoIterator>::Item;
    type IntoIter =
        Either<<&'data T as IntoIterator>::IntoIter, <&'data T as IntoParallelIterator>::Iter>;

    fn iter_with(&'data self, kind: IntoIterKind) -> Self::IntoIter {
        match kind {
            IntoIterKind::Sequential => Left(self.into_iter()),
            IntoIterKind::Parallel => Right(self.par_iter()),
        }
    }
}

pub trait MaybeParallelIterator {
    type Item: Send;
    type SeqIter: Iterator<Item = Self::Item>;
    type ParIter: ParallelIterator<Item = Self::Item>;

    fn map<B, F>(
        self,
        f: F,
    ) -> Either<std::iter::Map<Self::SeqIter, F>, rayon::iter::Map<Self::ParIter, F>>
    where
        F: Fn(Self::Item) -> B + Sync + Send,
        B: Send;

    fn filter<P>(
        self,
        predicate: P,
    ) -> Either<std::iter::Filter<Self::SeqIter, P>, rayon::iter::Filter<Self::ParIter, P>>
    where
        P: Fn(&Self::Item) -> bool + Sync + Send;

    #[allow(clippy::type_complexity)]
    fn flat_map<U, F>(
        self,
        f: F,
    ) -> Either<std::iter::FlatMap<Self::SeqIter, U, F>, rayon::iter::FlatMap<Self::ParIter, F>>
    where
        F: Fn(Self::Item) -> U + Sync + Send,
        U: IntoIterator + IntoParallelIterator;

    fn reduce<OP, ID>(self, identity: ID, op: OP) -> Self::Item
    where
        OP: Fn(Self::Item, Self::Item) -> Self::Item + Sync + Send,
        ID: Fn() -> Self::Item + Sync + Send;

    fn collect<C>(self) -> C
    where
        C: FromParallelIterator<Self::Item> + FromIterator<Self::Item>;
}

impl<Seq, Par> MaybeParallelIterator for Either<Seq, Par>
where
    Seq: Iterator,
    Seq::Item: Send,
    Par: ParallelIterator<Item = Seq::Item>,
{
    type Item = Seq::Item;
    type SeqIter = Seq;
    type ParIter = Par;

    fn map<B, F>(self, f: F) -> Either<std::iter::Map<Seq, F>, rayon::iter::Map<Par, F>>
    where
        F: Fn(Self::Item) -> B + Sync + Send,
        B: Send,
    {
        match self {
            Left(seq) => Left(seq.map(f)),
            Right(par) => Right(par.map(f)),
        }
    }

    fn filter<P>(
        self,
        predicate: P,
    ) -> Either<std::iter::Filter<Seq, P>, rayon::iter::Filter<Par, P>>
    where
        P: Fn(&Self::Item) -> bool + Sync + Send,
    {
        match self {
            Left(seq) => Left(seq.filter(predicate)),
            Right(par) => Right(par.filter(predicate)),
        }
    }

    fn flat_map<U, F>(
        self,
        f: F,
    ) -> Either<std::iter::FlatMap<Self::SeqIter, U, F>, rayon::iter::FlatMap<Self::ParIter, F>>
    where
        F: Fn(Self::Item) -> U + Sync + Send,
        U: IntoIterator + IntoParallelIterator,
    {
        match self {
            Left(seq) => Left(seq.flat_map(f)),
            Right(par) => Right(par.flat_map(f)),
        }
    }

    fn reduce<OP, ID>(self, identity: ID, op: OP) -> Self::Item
    where
        OP: Fn(Self::Item, Self::Item) -> Self::Item + Sync + Send,
        ID: Fn() -> Self::Item + Sync + Send,
    {
        match self {
            Left(seq) => seq.reduce(op).unwrap_or_else(identity),
            Right(par) => par.reduce(identity, op),
        }
    }

    fn collect<C>(self) -> C
    where
        C: FromParallelIterator<Self::Item> + FromIterator<Self::Item>,
    {
        match self {
            Left(seq) => seq.collect(),
            Right(par) => par.collect(),
        }
    }
}

pub mod prelude {
    pub use crate::iter::{
        IntoIterKind, IntoIteratorWith, IntoRefIteratorWith, MaybeParallelIterator,
    };
}

#[cfg(test)]
mod tests {
    use crate::iter::{IntoIterKind, IntoIteratorWith, IntoRefIteratorWith, MaybeParallelIterator};

    #[test]
    fn test_par() {
        let v = vec![1, 2, 3, 4, 5]
            .into_iter_with(IntoIterKind::Parallel)
            .map(|x| x * 2)
            .map(|x| x + 1)
            .collect::<Vec<_>>();

        assert_eq!(v, vec![3, 5, 7, 9, 11]);
    }

    #[test]
    fn test_par_reduce() {
        let v = vec![1, 2, 3, 4, 5]
            .into_iter_with(IntoIterKind::Parallel)
            .map(|x| x * 2)
            .map(|x| x + 1)
            .reduce(|| 0, |acc, x| acc + x);

        assert_eq!(v, 35);
    }

    #[test]
    fn test_seq() {
        let v = vec![1, 2, 3, 4, 5]
            .into_iter_with(IntoIterKind::Sequential)
            .map(|x| x * 2)
            .map(|x| x + 1)
            .collect::<Vec<_>>();

        assert_eq!(v, vec![3, 5, 7, 9, 11]);
    }

    #[test]
    fn test_seq_reduce() {
        let v = vec![1, 2, 3, 4, 5]
            .into_iter_with(IntoIterKind::Sequential)
            .map(|x| x * 2)
            .map(|x| x + 1)
            .reduce(|| 0, |acc, x| acc + x);

        assert_eq!(v, 35);
    }

    #[test]
    fn test_ref_par() {
        let v = vec![1, 2, 3, 4, 5]
            .iter_with(IntoIterKind::Parallel)
            .map(|x| x * 2)
            .map(|x| x + 1)
            .filter(|x| x > &3)
            .flat_map(|x| vec![x, x + 100])
            .collect::<Vec<_>>();

        assert_eq!(v, vec![5, 105, 7, 107, 9, 109, 11, 111]);
    }

    #[test]
    fn test_ref_seq() {
        let v = vec![1, 2, 3, 4, 5]
            .iter_with(IntoIterKind::Sequential)
            .map(|x| x * 2)
            .map(|x| x + 1)
            .filter(|x| x > &3)
            .flat_map(|x| vec![x, x + 100])
            .collect::<Vec<_>>();

        assert_eq!(v, vec![5, 105, 7, 107, 9, 109, 11, 111]);
    }
}
