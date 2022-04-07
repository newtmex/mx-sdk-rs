use crate::{
    interpret_trait::{InterpretableFrom, InterpreterContext, IntoRaw},
    model::{
        BigUintValue, BytesKey, BytesValue, CheckEsdtMap, CheckStorage, CheckStorageDetails,
        CheckValue, U64Value,
    },
    serde_raw::{CheckAccountRaw, CheckBytesValueRaw},
};

#[derive(Debug, Default)]
pub struct CheckAccount {
    pub comment: Option<String>,
    pub nonce: CheckValue<U64Value>,
    pub balance: CheckValue<BigUintValue>,
    pub esdt: CheckEsdtMap,
    pub username: CheckValue<BytesValue>,
    pub storage: CheckStorage,
    pub code: CheckValue<BytesValue>,
    pub async_call_data: CheckValue<BytesValue>,
}

impl CheckAccount {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn nonce<V>(mut self, nonce: V) -> Self
    where
        U64Value: InterpretableFrom<V>,
    {
        self.nonce = CheckValue::Equal(U64Value::interpret_from(
            nonce,
            &InterpreterContext::default(),
        ));
        self
    }

    pub fn balance<V>(mut self, balance_expr: V) -> Self
    where
        BigUintValue: InterpretableFrom<V>,
    {
        self.balance = CheckValue::Equal(BigUintValue::interpret_from(
            balance_expr,
            &InterpreterContext::default(),
        ));
        self
    }

    pub fn check_storage(mut self, key: &str, value: &str) -> Self {
        let mut details = match self.storage {
            CheckStorage::Star => CheckStorageDetails::default(),
            CheckStorage::Equal(details) => details,
        };
        details.storages.insert(
            BytesKey::interpret_from(key, &InterpreterContext::default()),
            CheckValue::Equal(BytesValue::interpret_from(
                value,
                &InterpreterContext::default(),
            )),
        );
        self.storage = CheckStorage::Equal(details);
        self
    }
}

impl InterpretableFrom<Box<CheckAccountRaw>> for CheckAccount {
    fn interpret_from(from: Box<CheckAccountRaw>, context: &InterpreterContext) -> Self {
        CheckAccount {
            comment: from.comment,
            nonce: CheckValue::<U64Value>::interpret_from(from.nonce, context),
            balance: CheckValue::<BigUintValue>::interpret_from(from.balance, context),
            esdt: CheckEsdtMap::interpret_from(from.esdt, context),
            username: CheckValue::<BytesValue>::interpret_from(from.username, context),
            storage: CheckStorage::interpret_from(from.storage, context),
            code: CheckValue::<BytesValue>::interpret_from(from.code, context),
            async_call_data: CheckValue::<BytesValue>::interpret_from(
                from.async_call_data,
                context,
            ),
        }
    }
}

impl IntoRaw<CheckAccountRaw> for CheckAccount {
    fn into_raw(self) -> CheckAccountRaw {
        CheckAccountRaw {
            comment: self.comment,
            nonce: self.nonce.into_raw(),
            balance: self.balance.into_raw(),
            esdt: self.esdt.into_raw(),
            username: self.username.into_raw(),
            storage: self.storage.into_raw(),
            code: self.code.into_raw(),
            owner: CheckBytesValueRaw::Unspecified,
            async_call_data: self.async_call_data.into_raw(),
        }
    }
}
