use crate::prelude::{ExternalNavigationFailure, GenericRouterContext, NavigationTarget, Routable};

/// A view into the navigation state of a router.
#[derive(Clone)]
pub struct GenericNavigator<R: Routable>(pub(crate) GenericRouterContext<R>);

impl<R: Routable> GenericNavigator<R> {
    /// Check whether there is a previous page to navigate back to.
    #[must_use]
    pub fn can_go_back(&self) -> bool {
        self.0.can_go_back()
    }

    /// Check whether there is a future page to navigate forward to.
    #[must_use]
    pub fn can_go_forward(&self) -> bool {
        self.0.can_go_forward()
    }

    /// Go back to the previous location.
    ///
    /// Will fail silently if there is no previous location to go to.
    pub fn go_back(&self) {
        self.0.go_back();
    }

    /// Go back to the next location.
    ///
    /// Will fail silently if there is no next location to go to.
    pub fn go_forward(&self) {
        self.0.go_forward();
    }

    /// Push a new location.
    ///
    /// The previous location will be available to go back to.
    pub fn push(
        &self,
        target: impl Into<NavigationTarget<R>>,
    ) -> Option<ExternalNavigationFailure> {
        self.0.push(target)
    }

    /// Replace the current location.
    ///
    /// The previous location will **not** be available to go back to.
    pub fn replace(
        &self,
        target: impl Into<NavigationTarget<R>>,
    ) -> Option<ExternalNavigationFailure> {
        self.0.replace(target)
    }
}
