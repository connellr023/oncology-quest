use super::tasks::{Supertask, Task};
use crate::utilities::parsables::SubtaskTitle;
use serde::{Serialize, Deserialize, Serializer, Deserializer};
use anyhow::anyhow;

/// A collection of supertasks representing the overall task entry structure.
#[derive(Serialize, Deserialize)]
pub struct Entries(Vec<Supertask>);

impl Entries {
    /// Gets a mutable reference to a supertask based on its index.
    /// 
    /// # Arguments
    /// 
    /// * `index` - The index of the supertask to retrieve.
    /// 
    /// # Returns
    /// 
    /// A result containing a mutable reference to the supertask if it exists.
    pub fn supertask(&mut self, index: usize) -> anyhow::Result<&mut Supertask> {
        match self.0.get_mut(index) {
            Some(supertask) => Ok(supertask),
            None => Err(anyhow!("Supertask index out of bounds"))
        }
    }

    /// Gets a mutable reference to a task based on its index.
    /// 
    /// # Arguments
    /// 
    /// * `index` - The index of the task to retrieve.
    /// 
    /// # Returns
    /// 
    /// A result containing a mutable reference to the task if it exists.
    pub fn task(&mut self, index: (usize, usize)) -> anyhow::Result<&mut Task> {
        let (supertask_index, task_index) = index;
        let supertask = self.supertask(supertask_index)?;

        match supertask.tasks.get_mut(task_index) {
            Some(task) => Ok(task),
            None => Err(anyhow!("Task index out of bounds"))
        }
    }

    /// Gets a mutable reference to a subtask title based on its index.
    /// 
    /// # Arguments
    /// 
    /// * `index` - The index of the subtask title to retrieve.
    /// 
    /// # Returns
    /// 
    /// A result containing a mutable reference to the subtask title if it exists.
    pub fn subtask_title(&mut self, index: (usize, usize, usize)) -> anyhow::Result<&mut SubtaskTitle> {
        let (supertask_index, task_index, subtask_index) = index;
        let task = self.task((supertask_index, task_index))?;

        match task.tasks.get_mut(subtask_index) {
            Some(subtask_title) => Ok(subtask_title),
            None => Err(anyhow!("Subtask index out of bounds"))
        }
    }

    /// Pushes a new supertask to the collection.
    /// 
    /// # Arguments
    /// 
    /// * `supertask` - The supertask to push.
    pub fn push_supertask(&mut self, supertask: Supertask) {
        self.0.push(supertask);
    }

    /// Pops the last supertask from the collection.
    /// 
    /// # Returns
    /// 
    /// A result containing the popped supertask if it exists.
    pub fn pop_supertask(&mut self) -> anyhow::Result<Supertask> {
        self.0.pop().ok_or(anyhow!("No supertasks to pop"))
    }

    /// Pushes a new task to a supertask.
    /// 
    /// # Arguments
    /// 
    /// * `supertask_index` - The index of the supertask to push the task to.
    /// 
    /// * `task` - The task to push.
    /// 
    /// # Returns
    /// 
    /// A result indicating whether the push operation was successful.
    pub fn push_task(&mut self, supertask_index: usize, task: Task) -> anyhow::Result<()> {
        let supertask = self.supertask(supertask_index)?;
        supertask.tasks.push(task);

        Ok(())
    }


    /// Pops the last task from a supertask.
    /// 
    /// # Arguments
    /// 
    /// * `supertask_index` - The index of the supertask to pop the task from.
    /// 
    /// # Returns
    /// 
    /// A result containing the popped task if it exists.
    pub fn pop_task(&mut self, supertask_index: usize) -> anyhow::Result<Task> {
        let supertask = self.supertask(supertask_index)?;
        supertask.tasks.pop().ok_or(anyhow!("No tasks to pop"))
    }

    /// Pushes a new subtask title to a task.
    /// 
    /// # Arguments
    /// 
    /// * `task_index` - The index of the task to push the subtask title to.
    /// * `subtask_title` - The subtask title to push.
    /// 
    /// # Returns
    /// 
    /// A result indicating whether the push operation was successful.
    pub fn push_subtask_title(&mut self, task_index: (usize, usize), subtask_title: SubtaskTitle) -> anyhow::Result<()> {
        let task = self.task(task_index)?;
        task.tasks.push(subtask_title);

        Ok(())
    }

    /// Pops the last subtask title from a task.
    /// 
    /// # Arguments
    /// 
    /// * `task_index` - The index of the task to pop the subtask title from.
    /// 
    /// # Returns
    /// 
    /// A result containing the popped subtask title if it exists.
    pub fn pop_subtask_title(&mut self, task_index: (usize, usize)) -> anyhow::Result<SubtaskTitle> {
        let task = self.task(task_index)?;
        task.tasks.pop().ok_or(anyhow!("No subtask titles to pop"))
    }
}

/// An index representing a location in the task entry structure.
#[derive(Debug)]
pub enum EntryIndex {
    Empty,
    SupertaskIndex(usize),
    TaskIndex(usize, usize),
    SubtaskIndex(usize, usize, usize)
}

impl Serialize for EntryIndex {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where S: Serializer {
        match self {
            Self::Empty => {
                <[usize]>::serialize(&[], serializer)
            },
            Self::SupertaskIndex(index) => {
                <[usize]>::serialize(&[*index], serializer)
            },
            Self::TaskIndex(supertask_index, task_index) => {
                <[usize]>::serialize(&[*supertask_index, *task_index], serializer)
            },
            Self::SubtaskIndex(supertask_index, task_index, subtask_index) => {
                <[usize]>::serialize(&[*supertask_index, *task_index, *subtask_index], serializer)
            }
        }
    }
}

impl<'de> Deserialize<'de> for EntryIndex {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where D: Deserializer<'de> {
        let index = Box::<[usize]>::deserialize(deserializer)?;

        match index.len() {
            0 => Ok(Self::Empty),
            1 => Ok(Self::SupertaskIndex(index[0])),
            2 => Ok(Self::TaskIndex(index[0], index[1])),
            3 => Ok(Self::SubtaskIndex(index[0], index[1], index[2])),
            _ => Err(serde::de::Error::custom("Invalid index length"))
        }
    }
}