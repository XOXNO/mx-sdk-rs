use crate::{
    api::ManagedTypeApi,
    typenum::Unsigned,
    types::{BigUint, Decimals, ManagedDecimalSigned, NumDecimals},
};

use core::ops::{Div, DivAssign, Sub};

use super::ConstDecimals;

impl<M: ManagedTypeApi, D1: Decimals, D2: Decimals> DivAssign<&ManagedDecimalSigned<M, D2>>
    for ManagedDecimalSigned<M, D1>
{
    fn div_assign(&mut self, rhs: &ManagedDecimalSigned<M, D2>) {
        self.data *= rhs.scaling_factor().as_big_int();
        self.data /= &rhs.data;
    }
}

impl<M: ManagedTypeApi, D1: Decimals, D2: Decimals> DivAssign<ManagedDecimalSigned<M, D2>>
    for ManagedDecimalSigned<M, D1>
{
    #[inline]
    fn div_assign(&mut self, rhs: ManagedDecimalSigned<M, D2>) {
        self.div_assign(&rhs);
    }
}

impl<M: ManagedTypeApi, D: Decimals> Div<NumDecimals> for ManagedDecimalSigned<M, D> {
    type Output = Self;

    fn div(self, other: NumDecimals) -> Self::Output {
        ManagedDecimalSigned {
            data: self.data / BigUint::from(other),
            decimals: self.decimals,
        }
    }
}

impl<M: ManagedTypeApi, D1: Decimals, D2: Decimals> Div<ManagedDecimalSigned<M, D2>>
    for ManagedDecimalSigned<M, D1>
where
    D1: Sub<D2>,
    <D1 as Sub<D2>>::Output: Decimals,
{
    type Output = ManagedDecimalSigned<M, <D1 as Sub<D2>>::Output>;

    fn div(self, other: ManagedDecimalSigned<M, D2>) -> Self::Output {
        ManagedDecimalSigned {
            data: self.data / other.data,
            decimals: self.decimals - other.decimals,
        }
    }
}

// var + const
impl<DECIMALS: Unsigned, M: ManagedTypeApi> Div<ManagedDecimalSigned<M, ConstDecimals<DECIMALS>>>
    for ManagedDecimalSigned<M, NumDecimals>
{
    type Output = ManagedDecimalSigned<M, NumDecimals>;

    fn div(self, rhs: ManagedDecimalSigned<M, ConstDecimals<DECIMALS>>) -> Self::Output {
        self / rhs.into_var_decimals()
    }
}

// const + var
impl<DECIMALS: Unsigned, M: ManagedTypeApi> Div<ManagedDecimalSigned<M, NumDecimals>>
    for ManagedDecimalSigned<M, ConstDecimals<DECIMALS>>
{
    type Output = ManagedDecimalSigned<M, NumDecimals>;

    fn div(self, rhs: ManagedDecimalSigned<M, NumDecimals>) -> Self::Output {
        self.into_var_decimals() / rhs
    }
}
