use std::sync::OnceLock;

use hudhook::tracing::error;

/// er-params.bin 中每条记录的磁盘布局（36 字节、无 padding）。
///
/// `[id: i32 LE][neutral .. dark: 8 × f32 LE]`
#[derive(Debug, Clone, Copy, Default)]
#[repr(C)]
pub(crate) struct DamageCutRate {
    pub(crate) id: i32,
    pub(crate) neutral_damage_cut_rate: f32,
    pub(crate) slash_damage_cut_rate: f32,
    pub(crate) blow_damage_cut_rate: f32,
    pub(crate) thrust_damage_cut_rate: f32,
    pub(crate) magic_damage_cut_rate: f32,
    pub(crate) fire_damage_cut_rate: f32,
    pub(crate) thunder_damage_cut_rate: f32,
    pub(crate) dark_damage_cut_rate: f32,
}

const ROW_SIZE: usize = 36;

/// 缓存整个文件；`None` 表示加载失败（已记录日志）。
static ER_PARAMS: OnceLock<Option<Vec<DamageCutRate>>> = OnceLock::new();

/// 读取 DLL 同目录下的 er-params.bin。仅在首次调用时执行。
fn load() -> Option<Vec<DamageCutRate>> {
    let path = crate::util::get_dll_path()?.parent()?.join("er-params.bin");

    let bytes = match std::fs::read(&path) {
        Ok(b) => b,
        Err(e) => {
            error!("无法读取 er-params.bin ({path:?}): {e}");
            return None;
        },
    };

    if bytes.len() % ROW_SIZE != 0 {
        error!("er-params.bin 长度 {} 不是 {ROW_SIZE} 的整数倍，拒绝加载", bytes.len());
        return None;
    }

    Some(
        bytes
            .as_chunks::<ROW_SIZE>()
            .0
            .iter()
            .map(|c| DamageCutRate {
                id: i32::from_le_bytes(c[0..4].try_into().unwrap()),
                neutral_damage_cut_rate: f32::from_le_bytes(c[4..8].try_into().unwrap()),
                slash_damage_cut_rate: f32::from_le_bytes(c[8..12].try_into().unwrap()),
                blow_damage_cut_rate: f32::from_le_bytes(c[12..16].try_into().unwrap()),
                thrust_damage_cut_rate: f32::from_le_bytes(c[16..20].try_into().unwrap()),
                magic_damage_cut_rate: f32::from_le_bytes(c[20..24].try_into().unwrap()),
                fire_damage_cut_rate: f32::from_le_bytes(c[24..28].try_into().unwrap()),
                thunder_damage_cut_rate: f32::from_le_bytes(c[28..32].try_into().unwrap()),
                dark_damage_cut_rate: f32::from_le_bytes(c[32..36].try_into().unwrap()),
            })
            .collect(),
    )
}

/// 按 ID 升序二分查找。文件未加载或 ID 未命中时返回 `None`。
pub(crate) fn lookup(param_id: u32) -> Option<&'static DamageCutRate> {
    let rows = ER_PARAMS.get_or_init(load).as_ref()?;
    let target = param_id as i32;

    match rows.binary_search_by(|r| r.id.cmp(&target)) {
        Ok(mut idx) => {
            // ID 假设唯一；若存在重复，回溯到首条。
            while idx > 0 && rows[idx - 1].id == target {
                idx -= 1;
            }
            Some(&rows[idx])
        },
        Err(_) => None,
    }
}
