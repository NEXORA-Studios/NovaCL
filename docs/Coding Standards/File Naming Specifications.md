## **文件命名规范**

1. **统一使用 PascalCase（大驼峰命名）**  
   每个单词首字母大写，例：`UserProfile.rs`、`LoginHandler.ts`

2. **不使用下划线（`_`）、中划线（`-`）**  
   避免混淆和不一致，保持风格统一。

3. **文件名应清晰描述内容职责**  
   ✅ 推荐：`AuthService.rs`  
   ❌ 避免：`Helper.rs`、`Utils.rs`、`Temp.rs`

4. **测试文件命名规则**  
   在原文件名后加 `Test`，如：

    - `UserService.rs` ➝ `UserServiceTest.rs`

5. **需要分开编写的组件命名规则**
   对于复杂的组件，确实有需要拆分成多个文件的情况，在原组件名字开头加 `_` 前缀，并在原组件名字后方加具体小组件名字。
   示例：`UserCard.vue`、`_UserCardHeader.vue`、`_UserCardBody.vue`、`_UserCardFooter.vue`

6. **组件或页面文件与其功能保持一致**（适用于前端项目）  
   示例：`UserCard.vue`、`DashboardLayout.ts`

7. **配置或常量文件也遵循 PascalCase**  
   如：`AppConfig.ts`、`GlobalConstants.ts`
