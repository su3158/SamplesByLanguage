const obj = {
    id: 1,
    name: 'foo'
};

const array = [
    { id: 2, name: 'bar' },
    { id: 3, name: 'hog' }
]

// ログ
console.log('this log');
// 情報
console.info('this info');
// 警告
console.warn('this warn');
// エラー
console.error('this error');
// オブジェクト
console.dir(obj);
// テーブル
console.table(array);