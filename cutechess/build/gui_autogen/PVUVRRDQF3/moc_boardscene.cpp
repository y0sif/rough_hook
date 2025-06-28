/****************************************************************************
** Meta object code from reading C++ file 'boardscene.h'
**
** Created by: The Qt Meta Object Compiler version 67 (Qt 5.15.13)
**
** WARNING! All changes made in this file will be lost!
*****************************************************************************/

#include <memory>
#include "../../../projects/gui/src/boardview/boardscene.h"
#include <QtCore/qbytearray.h>
#include <QtCore/qmetatype.h>
#if !defined(Q_MOC_OUTPUT_REVISION)
#error "The header file 'boardscene.h' doesn't include <QObject>."
#elif Q_MOC_OUTPUT_REVISION != 67
#error "This file was generated using the moc from 5.15.13. It"
#error "cannot be used with the include files from this version of Qt."
#error "(The moc has changed too much.)"
#endif

QT_BEGIN_MOC_NAMESPACE
QT_WARNING_PUSH
QT_WARNING_DISABLE_DEPRECATED
struct qt_meta_stringdata_BoardScene_t {
    QByteArrayData data[23];
    char stringdata0[244];
};
#define QT_MOC_LITERAL(idx, ofs, len) \
    Q_STATIC_BYTE_ARRAY_DATA_HEADER_INITIALIZER_WITH_OFFSET(len, \
    qptrdiff(offsetof(qt_meta_stringdata_BoardScene_t, stringdata0) + ofs \
        - idx * sizeof(QByteArrayData)) \
    )
static const qt_meta_stringdata_BoardScene_t qt_meta_stringdata_BoardScene = {
    {
QT_MOC_LITERAL(0, 0, 10), // "BoardScene"
QT_MOC_LITERAL(1, 11, 9), // "humanMove"
QT_MOC_LITERAL(2, 21, 0), // ""
QT_MOC_LITERAL(3, 22, 18), // "Chess::GenericMove"
QT_MOC_LITERAL(4, 41, 4), // "move"
QT_MOC_LITERAL(5, 46, 11), // "Chess::Side"
QT_MOC_LITERAL(6, 58, 4), // "side"
QT_MOC_LITERAL(7, 63, 8), // "populate"
QT_MOC_LITERAL(8, 72, 12), // "setFenString"
QT_MOC_LITERAL(9, 85, 9), // "fenString"
QT_MOC_LITERAL(10, 95, 8), // "makeMove"
QT_MOC_LITERAL(11, 104, 11), // "Chess::Move"
QT_MOC_LITERAL(12, 116, 8), // "undoMove"
QT_MOC_LITERAL(13, 125, 4), // "flip"
QT_MOC_LITERAL(14, 130, 14), // "onGameFinished"
QT_MOC_LITERAL(15, 145, 10), // "ChessGame*"
QT_MOC_LITERAL(16, 156, 4), // "game"
QT_MOC_LITERAL(17, 161, 13), // "Chess::Result"
QT_MOC_LITERAL(18, 175, 6), // "result"
QT_MOC_LITERAL(19, 182, 20), // "onTransitionFinished"
QT_MOC_LITERAL(20, 203, 17), // "onPromotionChosen"
QT_MOC_LITERAL(21, 221, 12), // "Chess::Piece"
QT_MOC_LITERAL(22, 234, 9) // "promotion"

    },
    "BoardScene\0humanMove\0\0Chess::GenericMove\0"
    "move\0Chess::Side\0side\0populate\0"
    "setFenString\0fenString\0makeMove\0"
    "Chess::Move\0undoMove\0flip\0onGameFinished\0"
    "ChessGame*\0game\0Chess::Result\0result\0"
    "onTransitionFinished\0onPromotionChosen\0"
    "Chess::Piece\0promotion"
};
#undef QT_MOC_LITERAL

static const uint qt_meta_data_BoardScene[] = {

 // content:
       8,       // revision
       0,       // classname
       0,    0, // classinfo
      10,   14, // methods
       0,    0, // properties
       0,    0, // enums/sets
       0,    0, // constructors
       0,       // flags
       1,       // signalCount

 // signals: name, argc, parameters, tag, flags
       1,    2,   64,    2, 0x06 /* Public */,

 // slots: name, argc, parameters, tag, flags
       7,    0,   69,    2, 0x0a /* Public */,
       8,    1,   70,    2, 0x0a /* Public */,
      10,    1,   73,    2, 0x0a /* Public */,
      10,    1,   76,    2, 0x0a /* Public */,
      12,    0,   79,    2, 0x0a /* Public */,
      13,    0,   80,    2, 0x0a /* Public */,
      14,    2,   81,    2, 0x0a /* Public */,
      19,    0,   86,    2, 0x08 /* Private */,
      20,    1,   87,    2, 0x08 /* Private */,

 // signals: parameters
    QMetaType::Void, 0x80000000 | 3, 0x80000000 | 5,    4,    6,

 // slots: parameters
    QMetaType::Void,
    QMetaType::Void, QMetaType::QString,    9,
    QMetaType::Void, 0x80000000 | 11,    4,
    QMetaType::Void, 0x80000000 | 3,    4,
    QMetaType::Void,
    QMetaType::Void,
    QMetaType::Void, 0x80000000 | 15, 0x80000000 | 17,   16,   18,
    QMetaType::Void,
    QMetaType::Void, 0x80000000 | 21,   22,

       0        // eod
};

void BoardScene::qt_static_metacall(QObject *_o, QMetaObject::Call _c, int _id, void **_a)
{
    if (_c == QMetaObject::InvokeMetaMethod) {
        auto *_t = static_cast<BoardScene *>(_o);
        (void)_t;
        switch (_id) {
        case 0: _t->humanMove((*reinterpret_cast< const Chess::GenericMove(*)>(_a[1])),(*reinterpret_cast< const Chess::Side(*)>(_a[2]))); break;
        case 1: _t->populate(); break;
        case 2: _t->setFenString((*reinterpret_cast< const QString(*)>(_a[1]))); break;
        case 3: _t->makeMove((*reinterpret_cast< const Chess::Move(*)>(_a[1]))); break;
        case 4: _t->makeMove((*reinterpret_cast< const Chess::GenericMove(*)>(_a[1]))); break;
        case 5: _t->undoMove(); break;
        case 6: _t->flip(); break;
        case 7: _t->onGameFinished((*reinterpret_cast< ChessGame*(*)>(_a[1])),(*reinterpret_cast< Chess::Result(*)>(_a[2]))); break;
        case 8: _t->onTransitionFinished(); break;
        case 9: _t->onPromotionChosen((*reinterpret_cast< const Chess::Piece(*)>(_a[1]))); break;
        default: ;
        }
    } else if (_c == QMetaObject::RegisterMethodArgumentMetaType) {
        switch (_id) {
        default: *reinterpret_cast<int*>(_a[0]) = -1; break;
        case 0:
            switch (*reinterpret_cast<int*>(_a[1])) {
            default: *reinterpret_cast<int*>(_a[0]) = -1; break;
            case 0:
                *reinterpret_cast<int*>(_a[0]) = qRegisterMetaType< Chess::GenericMove >(); break;
            case 1:
                *reinterpret_cast<int*>(_a[0]) = qRegisterMetaType< Chess::Side >(); break;
            }
            break;
        case 4:
            switch (*reinterpret_cast<int*>(_a[1])) {
            default: *reinterpret_cast<int*>(_a[0]) = -1; break;
            case 0:
                *reinterpret_cast<int*>(_a[0]) = qRegisterMetaType< Chess::GenericMove >(); break;
            }
            break;
        case 7:
            switch (*reinterpret_cast<int*>(_a[1])) {
            default: *reinterpret_cast<int*>(_a[0]) = -1; break;
            case 1:
                *reinterpret_cast<int*>(_a[0]) = qRegisterMetaType< Chess::Result >(); break;
            }
            break;
        }
    } else if (_c == QMetaObject::IndexOfMethod) {
        int *result = reinterpret_cast<int *>(_a[0]);
        {
            using _t = void (BoardScene::*)(const Chess::GenericMove & , const Chess::Side & );
            if (*reinterpret_cast<_t *>(_a[1]) == static_cast<_t>(&BoardScene::humanMove)) {
                *result = 0;
                return;
            }
        }
    }
}

QT_INIT_METAOBJECT const QMetaObject BoardScene::staticMetaObject = { {
    QMetaObject::SuperData::link<QGraphicsScene::staticMetaObject>(),
    qt_meta_stringdata_BoardScene.data,
    qt_meta_data_BoardScene,
    qt_static_metacall,
    nullptr,
    nullptr
} };


const QMetaObject *BoardScene::metaObject() const
{
    return QObject::d_ptr->metaObject ? QObject::d_ptr->dynamicMetaObject() : &staticMetaObject;
}

void *BoardScene::qt_metacast(const char *_clname)
{
    if (!_clname) return nullptr;
    if (!strcmp(_clname, qt_meta_stringdata_BoardScene.stringdata0))
        return static_cast<void*>(this);
    return QGraphicsScene::qt_metacast(_clname);
}

int BoardScene::qt_metacall(QMetaObject::Call _c, int _id, void **_a)
{
    _id = QGraphicsScene::qt_metacall(_c, _id, _a);
    if (_id < 0)
        return _id;
    if (_c == QMetaObject::InvokeMetaMethod) {
        if (_id < 10)
            qt_static_metacall(this, _c, _id, _a);
        _id -= 10;
    } else if (_c == QMetaObject::RegisterMethodArgumentMetaType) {
        if (_id < 10)
            qt_static_metacall(this, _c, _id, _a);
        _id -= 10;
    }
    return _id;
}

// SIGNAL 0
void BoardScene::humanMove(const Chess::GenericMove & _t1, const Chess::Side & _t2)
{
    void *_a[] = { nullptr, const_cast<void*>(reinterpret_cast<const void*>(std::addressof(_t1))), const_cast<void*>(reinterpret_cast<const void*>(std::addressof(_t2))) };
    QMetaObject::activate(this, &staticMetaObject, 0, _a);
}
QT_WARNING_POP
QT_END_MOC_NAMESPACE
