use crate::{ConnectionError, NextEvent};

use tokio::runtime::Runtime;

///  MQTT connection. Maintains all the necessary state
pub struct Connection<E: NextEvent> {
    pub eventloop: E,
    runtime: Option<Runtime>,
}

impl<E: NextEvent> Connection<E> {
    pub fn new(eventloop: E, runtime: Runtime) -> Connection<E> {
        Connection {
            eventloop,
            runtime: Some(runtime),
        }
    }

    /// Returns an iterator over this connection. Iterating over this is all that's
    /// necessary to make connection progress and maintain a robust connection.
    /// Just continuing to loop will reconnect
    /// **NOTE** Don't block this while iterating
    #[must_use = "Connection should be iterated over a loop to make progress"]
    pub fn iter(&mut self) -> Iter<E> {
        let runtime = self.runtime.take();
        Iter {
            connection: self,
            runtime,
        }
    }
}

/// Iterator which polls the eventloop for connection progress
pub struct Iter<'a, E: NextEvent> {
    connection: &'a mut Connection<E>,
    runtime: Option<Runtime>,
}

impl<'a, E: NextEvent> Iterator for Iter<'a, E> {
    type Item = Result<E::Output, ConnectionError>;

    fn next(&mut self) -> Option<Self::Item> {
        match self
            .runtime
            .as_mut()?
            .block_on(self.connection.eventloop.next())
        {
            Ok(v) => Some(Ok(v)),
            // closing of request channel should stop the iterator
            Err(ConnectionError::RequestsDone) => {
                trace!("Done with requests");
                None
            }
            Err(ConnectionError::Cancel) => {
                trace!("Cancellation request received");
                None
            }
            Err(e) => Some(Err(e)),
        }
    }
}

impl<'a, E: NextEvent> Drop for Iter<'a, E> {
    fn drop(&mut self) {
        // TODO: Don't create new runtime in drop;
        self.connection.runtime = self.runtime.take();
    }
}
