use serde_json::Value;

pub struct Permission;

impl Permission {
    // 检查用户是否有特定权限
    pub fn has_permission(user_permissions: &str, required_permission: &str) -> bool {
        println!("Checking permission: {} against {}", required_permission, user_permissions);
        
        // 解析权限JSON
        let permissions: Value = match serde_json::from_str(user_permissions) {
            Ok(p) => p,
            Err(e) => {
                println!("Failed to parse permissions JSON: {}", e);
                return false;
            }
        };
        
        if let Some(perms) = permissions.as_array() {
            // 检查是否有超级权限 "*"
            if perms.iter().any(|p| p.as_str() == Some("*")) {
                println!("User has super permission *");
                return true;
            }
            
            // 检查具体权限
            let has_perm = perms.iter().any(|p| p.as_str() == Some(required_permission));
            println!("Permission check result: {}", has_perm);
            has_perm
        } else {
            println!("Permissions is not an array");
            false
        }
    }

    // 获取权限列表
    pub fn get_permissions(user_permissions: &str) -> Vec<String> {
        let permissions: Value = serde_json::from_str(user_permissions).unwrap_or_default();
        
        if let Some(perms) = permissions.as_array() {
            perms.iter()
                .filter_map(|p| p.as_str())
                .map(|s| s.to_string())
                .collect()
        } else {
            vec![]
        }
    }
} 