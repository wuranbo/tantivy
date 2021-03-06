use time::PreciseTime;

pub struct OpenTimer<'a> {
    name: &'static str,
    timer_tree: &'a mut TimerTree,
    start: PreciseTime,
    depth: u32,
}

impl<'a> OpenTimer<'a> {
    /// Starts timing a new named subtask
    ///
    /// The timer is stopped automatically 
    /// when the `OpenTimer` is dropped.
    pub fn open(&mut self, name: &'static str) -> OpenTimer {
        OpenTimer {
            name: name,
            timer_tree: self.timer_tree,
            start: PreciseTime::now(),
            depth: self.depth + 1,
        }
    }
}

impl<'a> Drop for OpenTimer<'a> {
    fn drop(&mut self,) {
        self.timer_tree.timings.push(Timing     {
            name: self.name,
            duration: self.start.to(PreciseTime::now()).num_microseconds().unwrap(),
            depth: self.depth,
        });
    }
}

/// Timing recording
#[derive(Debug, Serialize)]
pub struct Timing {
    name: &'static str,
    duration: i64,
    depth: u32,
}

/// Timer tree
#[derive(Debug, Serialize)]
pub struct TimerTree {
    timings: Vec<Timing>,
}

impl TimerTree {
        
    /// Returns the total time elapsed in microseconds 
    pub fn total_time(&self,) -> i64 {
        self.timings.last().unwrap().duration
    }
    
    /// Open a new named subtask
    pub fn open(&mut self, name: &'static str) -> OpenTimer {
        OpenTimer {
            name: name,
            timer_tree: self,
            start: PreciseTime::now(),
            depth: 0,
        }
    }
}

impl Default for TimerTree {
    fn default() -> TimerTree {
        TimerTree {
            timings: Vec::new(),
        }
    }
}


#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_timer() {
        let mut timer_tree = TimerTree::default();
        {
            let mut a = timer_tree.open("a");
            {
                let mut ab = a.open("b");
                {
                    let _abc = ab.open("c");
                }
                {
                    let _abd = ab.open("d");
                }
            }
        }
        assert_eq!(timer_tree.timings.len(), 4);
    }
}
