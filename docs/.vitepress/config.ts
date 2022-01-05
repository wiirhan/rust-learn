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
        },
    ]
}

