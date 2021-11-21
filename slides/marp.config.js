module.exports = {
    allowLocalFiles: true,
    ogImage: process.env.URL && `${process.env.URL}/og-image.jpg`,
    themeSet: 'themes',
    //url: process.env.URL,
    engine: ({ marp }) => marp.use(require('markdown-it-plantuml')),
  }