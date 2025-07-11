<template>
  <div class="dashboard">
    <h1>仪表盘</h1>
    <a-row :gutter="16">
      <a-col :span="6">
        <a-card>
          <a-statistic
            title="用户总数"
            :value="1234"
            :value-style="{ color: '#3f8600' }"
          />
        </a-card>
      </a-col>
      <a-col :span="6">
        <a-card>
          <a-statistic
            title="商品总数"
            :value="567"
            :value-style="{ color: '#1890ff' }"
          />
        </a-card>
      </a-col>
      <a-col :span="6">
        <a-card>
          <a-statistic
            title="订单总数"
            :value="890"
            :value-style="{ color: '#cf1322' }"
          />
        </a-card>
      </a-col>
      <a-col :span="6">
        <a-card>
          <a-statistic
            title="今日销售额"
            :value="12345.67"
            prefix="¥"
            :precision="2"
            :value-style="{ color: '#722ed1' }"
          />
        </a-card>
      </a-col>
    </a-row>
  </div>
</template>

<script setup>
// 仪表盘页面
</script>

<style scoped>
.dashboard {
  padding: 0;
}
</style> 