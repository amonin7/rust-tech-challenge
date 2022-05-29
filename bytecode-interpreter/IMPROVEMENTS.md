# Possible improvements, could be done with the code

Suppose we added the following bytecode instructions to our language:
1. `SEND_CHANNEL`: Pops the channel and a value from the stack and send the value on the channel using a blocking send
2. `RECV_CHANNEL`: Pops the channel from the stack, receives a value from the channel (this may block),
   and push the resulting value back onto the stack
3. `SPAWN`: Pop two functions from the stack and spawn them as concurrent tasks

Describe in a few sentences how each bytecode instruction could be interpreted,
and how your interpreter or language runtime could deal with the blocking nature of the `send` and the `receive` instructions.

## Answer

In the Rust, we can choose between using the _default_ channel,
that comes with a `Sender` with an **unlimited buffer** and hence will never block sending,
and the _sync_channel_ that comes with a `SyncSender` that will block sending if the buffer is full.

Both ends of a Rust channel can only be owned by one thread at the time,
however the `sender` half can be cloned, and through such cloning the conceptual
`sender` can be shared among threads. By this was we can make the "multi-producer, single-consumer".

So to get rid of blocks, occurred due to waiting for the receive operation, I think the easiest way is : \
Save in some shared buffer (queue, which is FIFO) threads (like their identifiers), which do want to receive the message.
And give each thread some delta of time to try to receive message. If the time is over and the thread did not receive a message yet,
we give a try to the next thread in our "receiver's queue".

A `Spawn` can be either polled for completion or execution of the current thread can be blocked indefinitely until a notification arrives.
This can be used with either futures or streams, with different methods being available on Spawn depending on which is used.
So I think to get rid of blockers over there, we could just pick the first version of `Spawn` implementation.
