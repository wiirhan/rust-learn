export default {
    title: 'Rust learn',
    description: 'My Rust learn note.',

    themeConfig: {
        editLinks: true,
        editLinkText: '在 GitHub 上编辑此页',
        lastUpdated: '上次更新',

        sidebar: {
            '/': getGuideSidebar()
        }
    }
}

function getGuideSidebar() {
    return [
        {
            text: 'Learn',
            children: [
                {text: 'Getting Started', link: '/'},
                {text: 'Tutorial Game', link: '/tutorial-game'},
            ]
        },{
            text: '通用编程概念',
            children: [
                {text: '变量和可变性', link: '/common-programming-concepts/variables-and-mutability'},
                {text: '数据类型', link: '/common-programming-concepts/data-types'},
                {text: '函数', link: '/common-programming-concepts/functions'},
            ]
        },{
            text: '所有权',
            children: [
                {text: '所有权', link: '/ownership/'},
                {text: '引用与借用', link: '/ownership/references-and-borrowing'},
            ]
        },
    ]
}

