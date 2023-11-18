pub trait Runner {
    type ActualState;
    type Result;
    type ExpectedState;
    type Error;

    fn init(&mut self, expected: &Self::ExpectedState) -> Result<Self::ActualState, Self::Error>;

    fn step(
        &mut self,
        actual: &mut Self::ActualState,
        expected: &Self::ExpectedState,
    ) -> Result<Self::Result, Self::Error>;

    fn result_invariant(
        &self,
        result: &Self::Result,
        expected: &Self::ExpectedState,
    ) -> Result<bool, Self::Error>;

    fn state_invariant(
        &self,
        actual: &Self::ActualState,
        expected: &Self::ExpectedState,
    ) -> Result<bool, Self::Error>;

    fn test(&mut self, trace: &[Self::ExpectedState]) -> Result<(), Self::Error> {
        if let Some(expected_init) = trace.first() {
            println!("🟢 step: initial");
            let mut actual = self.init(expected_init)?;
            assert!(
                self.state_invariant(&actual, expected_init)?,
                "🔴 state invariant failed at initialization"
            );
            for (i, expected) in trace.iter().enumerate().skip(1) {
                println!("🟢 step: {}", i);
                let result = self.step(&mut actual, expected)?;
                assert!(
                    self.result_invariant(&result, expected)?,
                    "🔴 result invariant failed at step {}",
                    i
                );
                assert!(
                    self.state_invariant(&actual, expected)?,
                    "🔴 state invariant failed at step {}",
                    i
                );
            }
        }

        Ok(())
    }
}
