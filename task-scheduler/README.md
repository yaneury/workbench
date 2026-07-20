# Task Scheduler

A Rust library able to schedule tasks (callable). Library optimizes for ergonomics and precision.
Supported operations are the following:

1) A task can be scheduled at any point during process' lifecycle. Either `cadence` or `instant` is
can be provided. The former is used for tasks that run at regular intervals. The latter is used for
one-shot task that run once at a specific point in time.
2) Prior to a task run, it can be descheduled, ensuring that the next run does NOT execute.

Guarantees provided by this library are:

1) Each task will RUN at the specified time with up to 1 millisecond.
2) No task blocks the execution of another concurrent task.
3) If either guarantee can't be met, then task is not scheduled and caller is informed immediately.

## Design

API is fairly simple:

```
struct TaskScheduler {
  ...
}

enum Schedule {
  Once(std::time::Instant), // Accurate down to 1ms
  Repeating(std::time::Duration) // Accurate down to 1ms
}

struct TaskHandle {}

impl TaskScheduler {
  fn schedule(f: Box<dyn Fn()>, schedule: Schedule) -> Option<TaskHandle>
  fn deschedule(handle: TaskHandle)
}
```


TaskHandle is unique identifier to a task. It used to deschedule so that the internal runtime knows
which task to remove from its tracker. A handle is guaranteed to be unique throughout the entire
lifetime of the process.

## Runtime
In order to meet the timing guarantee of 1 millisecond SLA and execution promise of no call being
blocked, the runtime powering this API is as such:

- Tasks run in dedicated threads, ensuring that they don't block other tasks. 
- To minimize the overhead of creating and destroying threads, the runtime will maintain a thread pool.
  One thing to keep in mind here, is that in order to fulfill the second guarantee, we must have N
  threads running where N is the number of tasks scheduled to execute at the same time. Now, at its
  extreme, we can have hundreds of threads, reaching the limits of what the kernel allows. Adding
  more and more threads will eventually lead to degradation of the timing precision since the kernel
  will be overburdened with time slicing a large number of threads with same priority.
- To wit, the runtime will accept a thread pool limit as a parameter, but will not enforce a cap on
  that value. It's up to callers to profile the library against benchmarks to know what the right
  limit is for the internal thread pool.
- Furthermore, the runtime will keep track of which tasks are scheduled to run and optimize it such
  that we use as little threads as possible, maximizing time slices for threads with active tasks.
- Tasks are stored in a min heap, ordered by their due time. Runtime maintains one thread for
  looping and scheduling tasks.
- Threads reserved are 1) API thread to handle calls to |schedule| and |deschedule| and 2) loop
  thread to pop next set of tasks and hand them off to thread, per tick.
 
### API Thread
The API thread, or "main thread" of the runtime, is primarily in charge of fulfilling caller
requests and informing the loop thread of when to wake up.

### Loop Thread
This thread will run a loop in at most 1kHz. However, we don't want to make excessive syscalls to
yield (read: sleep) to accomodate this. Instead, we will leverage facts about the task schedule to
sleep for longer periods of time if possible. The following will be enable this:

- When there is no task due, this thread will sleep for duration remaining for the "top" of the heap's
  due date.
- Or, if woken up in the event of a new task being scheduled or descheduled.
- Or, if woken up because a task completed.
- Whichever comes first.

The loop runs indefinitely until signaled to shutdown by API thread.

### Threadpool

The library will have its own thread pool implementation. Unlike most thread pools that use work
stealing policies in order to accomodate unbounded number of tasks, this one rejects tasks outright
if the number of request tasks is greater than the number of threads reserved for the pool. This
is the only way we can guarantee that requests are fulfilled within the promised SLA.

### Context

The runtime will communicate with callers via a signal surface that is exercised in their callbacks.
Each time a task is run, a context object will be provided. This contains optional metadata that
callables can use if they need:

- Current time: The time this task was scheduled.
- Counter: For repeating jobs, the number of times this job has run. First run will show #1.

Furthermore, the context object will be mutable such that a task can pass arbitrary values to
registered listeners (see next sub-section). These values will be keyed by strings. The underlying
key-value store is unique to each task such that different tasks can use same keys with no conflicts.

### Listener

Scheduler allows listeners to observe the status of scheduled tasks. When you schedule a task, a
Handle object is minted that uniquely identifies the task to the underlying runtime. This same
handle is used to registered / deregister a listener.


## Testing
The library ought to be tested against multiple fronts to ensure robustness:

1) **Correction**. First and foremost, the runtime must schedule jobs at the time determined by caller
   every time. No exception.
2) **Capacity**. The runtime should handle a large number of concurrent tasks running at any point and
   even larger number of tasks scheduled.
3) **Unpredictability**. The runtime should be amenable to unbounded, unpredictable usages.
4) **Compatibility**. The runtime should be operating system agnostic. That is to say, the tests
   should yield the same result regardless what platform it's running on.
