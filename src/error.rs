/// Union of all possibles errors that might occurs within Olympus.
#[derive(Debug, Clone, Copy)]
#[allow(non_camel_case_types)]
pub enum OlympusError {

    /// Error that happens within [KWindow]. 
    KWindow(KWindowError),

    /// Error that happens within [KEventDispatcherError]. 
    KEventDispatcher(KEventDispatcherError),
    
    /// Error that happens within [KAssetBroker].
    KAssetBroker(KAssetBrokerError),
}

/// Enumeration of possible [KWindow] errors.
#[derive(Debug, Clone, Copy)]
pub enum KWindowError {
    /// Happens when a window manager is not supported.
    NotSupported,

    /// Happens when no display server is found.
    NoDisplayServer,

    /// Happens when trying to resize a [KWindow] outside of allowed boundaries.
    SizeError,

    /// Happens when trying get hardware screen details failed.
    ScreenDetailError,
}

/// Enumeration of possible [KEventDispatcher] errors.
#[derive(Debug, Clone, Copy)]
pub enum KEventDispatcherError {
    /// Happens when trying to add the same [KEventReceiver] twice to a [KWindow].
    ReceiverAlreadyExists,

    /// Happens when trying to remove a [KEventReceiver] not added to a [KWindow].
    ReceiverNotFound,
}

/// Enumeration of possible errors that can happens within [KAssetBroker].
#[derive(Debug, Clone, Copy)]
pub enum KAssetBrokerError {
    /// Happens when a [KAssetSource] is not found within the broker.
    SourceNotFound,

    /// Happens when adding the same [KAssetSource] twice to the broker.
    SourceAlreadyExists,

    /// Happens when new priority set for [KAssetSource] is higher then the length of sources.
    PriorityOutOfBound,
}