use std::collections::HashSet;

pub struct Permission;

impl Permission {
    /// 检查用户是否有特定权限
    pub fn has_permission(user_permissions: &str, required_permission: &str) -> bool {
        println!("Checking permission: {} against {}", required_permission, user_permissions);
        
        // 如果用户有所有权限
        if user_permissions.contains("*") {
            return true;
        }
        
        // 解析权限JSON
        if let Ok(permissions) = serde_json::from_str::<Vec<String>>(user_permissions) {
            // 检查是否有匹配的权限
            permissions.contains(&required_permission.to_string())
        } else {
            false
        }
    }
    
    /// 检查用户是否有任意一个权限
    pub fn has_any_permission(user_permissions: &str, required_permissions: &[&str]) -> bool {
        for permission in required_permissions {
            if Self::has_permission(user_permissions, permission) {
                return true;
            }
        }
        false
    }

    /// 获取用户所有权限
    pub fn get_permissions(permissions_json: &str) -> HashSet<String> {
        let mut permissions = HashSet::new();
        
        if let Ok(perms) = serde_json::from_str::<Vec<String>>(permissions_json) {
            for perm in perms {
                if perm == "*" {
                    // 超级管理员拥有所有权限
                    permissions.insert("dashboard:read".to_string());
                    permissions.insert("user:read".to_string());
                    permissions.insert("user:write".to_string());
                    permissions.insert("product:read".to_string());
                    permissions.insert("product:write".to_string());
                    permissions.insert("order:read".to_string());
                    permissions.insert("order:write".to_string());
                    permissions.insert("category:read".to_string());
                    permissions.insert("category:write".to_string());
                    permissions.insert("settings:read".to_string());
                    permissions.insert("settings:write".to_string());
                    permissions.insert("logs:read".to_string());
                } else {
                    permissions.insert(perm);
                }
            }
        }
        
        permissions
    }
} 