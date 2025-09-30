use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};
use std::thread;
use std::time::Duration;

/// Snowflake ID 生成器
/// 结构：1位符号位 + 41位时间戳 + 10位机器ID + 12位序列号
pub struct Snowflake {
    machine_id: u64,
    sequence: AtomicU64,
    last_timestamp: AtomicU64,
}

impl Snowflake {
    const SEQUENCE_BITS: u64 = 12;
    const MACHINE_ID_BITS: u64 = 10;
    //const TIMESTAMP_BITS: u64 = 41;

    const MAX_SEQUENCE: u64 = (1 << Self::SEQUENCE_BITS) - 1;
    const MAX_MACHINE_ID: u64 = (1 << Self::MACHINE_ID_BITS) - 1;

    const MACHINE_ID_SHIFT: u64 = Self::SEQUENCE_BITS;
    const TIMESTAMP_SHIFT: u64 = Self::SEQUENCE_BITS + Self::MACHINE_ID_BITS;

    /// 创建一个新的 Snowflake 实例
    ///
    /// # 参数
    /// - `machine_id`: 机器ID (0-1023)
    ///
    /// # 示例
    /// ```
    /// let generator = Snowflake::new(1).unwrap();
    /// ```
    pub fn new(machine_id: u64) -> Result<Self, &'static str> {
        if machine_id > Self::MAX_MACHINE_ID {
            return Err("Machine ID 超出范围 (0-1023)");
        }

        Ok(Snowflake {
            machine_id,
            sequence: AtomicU64::new(0),
            last_timestamp: AtomicU64::new(0),
        })
    }

    /// 生成下一个 ID
    pub fn next_id(&self) -> u64 {
        let mut timestamp = self.current_timestamp();
        let mut sequence = self.sequence.load(Ordering::Relaxed);

        loop {
            let last_timestamp = self.last_timestamp.load(Ordering::Relaxed);

            if timestamp < last_timestamp {
                // 时钟回拨，等待时钟追上
                timestamp = self.wait_until_last_timestamp(last_timestamp);
                sequence = 0;
            } else if timestamp == last_timestamp {
                // 同一毫秒内，增加序列号
                sequence = (sequence + 1) & Self::MAX_SEQUENCE;
                if sequence == 0 {
                    // 序列号用尽，等待下一毫秒
                    timestamp = self.wait_next_millis(last_timestamp);
                }
            } else {
                // 新的毫秒，重置序列号
                sequence = 0;
            }

            // 尝试更新时间戳和序列号
            if self.last_timestamp
                .compare_exchange(
                    last_timestamp,
                    timestamp,
                    Ordering::AcqRel,
                    Ordering::Relaxed
                )
                .is_ok()
            {
                self.sequence.store(sequence, Ordering::Relaxed);
                break;
            }
        }

        (timestamp << Self::TIMESTAMP_SHIFT)
            | (self.machine_id << Self::MACHINE_ID_SHIFT)
            | sequence
    }

    /// 获取当前时间戳（毫秒）
    fn current_timestamp(&self) -> u64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("时钟异常")
            .as_millis() as u64
    }

    /// 等待直到超过指定的时间戳
    fn wait_until_last_timestamp(&self, last_timestamp: u64) -> u64 {
        let mut timestamp = self.current_timestamp();
        while timestamp <= last_timestamp {
            thread::sleep(Duration::from_millis(1));
            timestamp = self.current_timestamp();
        }
        timestamp
    }

    /// 等待下一毫秒
    fn wait_next_millis(&self, last_timestamp: u64) -> u64 {
        let mut timestamp = self.current_timestamp();
        while timestamp <= last_timestamp {
            thread::sleep(Duration::from_millis(1));
            timestamp = self.current_timestamp();
        }
        timestamp
    }

    /// 解析 ID 的各个部分
    pub fn parse_id(&self, id: u64) -> (u64, u64, u64) {
        let timestamp = id >> Self::TIMESTAMP_SHIFT;
        let machine_id = (id >> Self::MACHINE_ID_SHIFT) & Self::MAX_MACHINE_ID;
        let sequence = id & Self::MAX_SEQUENCE;

        (timestamp, machine_id, sequence)
    }
}

/// 线程安全的 Snowflake 生成器
//pub type SharedSnowflake = Arc<Snowflake>;

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;
    use std::sync::Arc;
    use std::thread;

    #[test]
    fn test_snowflake_basic() {
        let snowflake = Snowflake::new(1).unwrap();
        let id1 = snowflake.next_id();
        let id2 = snowflake.next_id();

        assert!(id1 < id2, "ID 应该单调递增");

        let (timestamp, machine_id, sequence) = snowflake.parse_id(id1);
        assert_eq!(machine_id, 1);
        assert!(timestamp > 0);
    }

    #[test]
    fn test_snowflake_uniqueness() {
        let snowflake = Arc::new(Snowflake::new(2).unwrap());
        let mut handles = vec![];
        let id_set = Arc::new(std::sync::Mutex::new(HashSet::new()));

        // 多线程生成 ID 测试唯一性
        for _ in 0..10 {
            let snowflake_clone = Arc::clone(&snowflake);
            let set_clone = Arc::clone(&id_set);

            handles.push(thread::spawn(move || {
                for _ in 0..100 {
                    let id = snowflake_clone.next_id();
                    set_clone.lock().unwrap().insert(id);
                }
            }));
        }

        for handle in handles {
            handle.join().unwrap();
        }

        assert_eq!(id_set.lock().unwrap().len(), 1000);
    }

    #[test]
    fn test_machine_id_validation() {
        assert!(Snowflake::new(1024).is_err());
        assert!(Snowflake::new(1023).is_ok());
    }
}
