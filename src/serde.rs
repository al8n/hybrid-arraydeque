use core::{fmt, marker::PhantomData};

use serde_core::{
  Deserialize, Deserializer, Serialize, Serializer,
  de::{Error, SeqAccess, Visitor},
};

use super::{ArrayDeque, ArraySize};

impl<T: Serialize, N: ArraySize> Serialize for ArrayDeque<T, N> {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    serializer.collect_seq(self)
  }
}

impl<'de, T: Deserialize<'de>, N: ArraySize> Deserialize<'de> for ArrayDeque<T, N> {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: Deserializer<'de>,
  {
    struct SeqVisitor<T, N: ArraySize> {
      marker: PhantomData<(T, N)>,
    }

    impl<'de, T, N: ArraySize> Visitor<'de> for SeqVisitor<T, N>
    where
      T: Deserialize<'de>,
    {
      type Value = ArrayDeque<T, N>;

      fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("a sequence")
      }

      #[inline]
      fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
      where
        A: SeqAccess<'de>,
      {
        let mut values = ArrayDeque::<T, N>::new();

        while let Some(value) = seq.next_element()? {
          if values.push_back(value).is_some() {
            return Err(Error::custom("sequence length exceeds capacity"));
          }
        }

        Ok(values)
      }
    }

    let visitor = SeqVisitor {
      marker: PhantomData,
    };
    deserializer.deserialize_seq(visitor)
  }

  fn deserialize_in_place<D>(deserializer: D, place: &mut Self) -> Result<(), D::Error>
  where
    D: Deserializer<'de>,
  {
    struct SeqInPlaceVisitor<'a, T, N: ArraySize>(&'a mut ArrayDeque<T, N>);

    impl<'de, T, N: ArraySize> Visitor<'de> for SeqInPlaceVisitor<'_, T, N>
    where
      T: Deserialize<'de>,
    {
      type Value = ();

      fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("a sequence")
      }

      #[inline]
      fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
      where
        A: SeqAccess<'de>,
      {
        self.0.clear();

        while let Some(value) = seq.next_element()? {
          if self.0.push_back(value).is_some() {
            return Err(Error::custom("sequence length exceeds capacity"));
          }
        }

        Ok(())
      }
    }

    deserializer.deserialize_seq(SeqInPlaceVisitor(place))
  }
}
