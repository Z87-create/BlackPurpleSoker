let token = "";

// 登录函数
async function login() {
    let u = document.getElementById("username").value.trim();
    let p = document.getElementById("password").value.trim();
    
    if (!u || !p) {
        alert("请输入用户名和密码！");
        return;
    }

    try {
        // 适配后端API路径，本地调试用相对路径
        let r = await fetch("/api/login", {
            method: "POST",
            headers: { "Content-Type": "application/json" },
            body: JSON.stringify({ username: u, password: p })
        });
        
        if (!r.ok) throw new Error("登录请求失败");
        let d = await r.json();
        
        if (d.code === 200) {
            token = d.data;
            document.getElementById("loginCard").style.display = "none";
            document.getElementById("mainContainer").style.display = "block";
            // 登录成功后默认显示AI面板
            showPanel('ai');
        } else {
            alert(`登录失败：${d.msg || "用户名或密码错误"}`);
        }
    } catch (error) {
        alert(`登录异常：${error.message}`);
    }
}

// 通用请求函数（带token鉴权）
async function req(url, opt = {}) {
    if (!token) {
        alert("请先登录！");
        document.getElementById("loginCard").style.display = "block";
        document.getElementById("mainContainer").style.display = "none";
        return { code: 401, msg: "未登录" };
    }

    try {
        let res = await fetch(url, {
            ...opt,
            headers: {
                "Content-Type": "application/json",
                "Authorization": "Bearer " + token,
                ...opt.headers
            }
        });
        
        if (!res.ok) throw new Error(`请求失败：${res.status}`);
        return await res.json();
    } catch (error) {
        alert(`请求异常：${error.message}`);
        return { code: 500, msg: error.message };
    }
}

// AI问答
async function askAI() {
    let q = document.getElementById("question").value.trim();
    if (!q) {
        alert("请输入问题后再提交！");
        return;
    }

    document.getElementById("aiResult").innerText = "正在查询中...";
    
    let d = await req("/api/ai", {
        method: "POST",
        body: JSON.stringify({ question: q })
    });
    
    document.getElementById("aiResult").innerText = d.data || d.msg;
}

// 补全：面板切换函数（HTML里的tab-btn点击调用）
function showPanel(panel) {
    // 隐藏所有面板
    const panels = document.querySelectorAll(".panel");
    panels.forEach(p => p.classList.remove("active"));
    
    // 移除所有按钮激活状态
    const tabs = document.querySelectorAll(".tab-btn");
    tabs.forEach(t => t.classList.remove("active"));
    
    // 显示目标面板，激活对应按钮
    if (panel === 'ai') {
        document.getElementById("aiPanel").classList.add("active");
        tabs[0].classList.add("active");
    } else if (panel === 'kb') {
        document.getElementById("kbPanel").classList.add("active");
        tabs[1].classList.add("active");
        // 自动加载知识库
        showKnowledge();
    } else if (panel === 'excel') {
        document.getElementById("excelPanel").classList.add("active");
        tabs[2].classList.add("active");
    }
}

// 显示AI面板（兼容旧逻辑）
function showAi() {
    showPanel('ai');
}

// 页面加载完成后，默认聚焦用户名输入框
document.addEventListener("DOMContentLoaded", () => {
    document.getElementById("username").focus();
});