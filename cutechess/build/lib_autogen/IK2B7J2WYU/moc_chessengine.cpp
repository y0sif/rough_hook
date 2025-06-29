/****************************************************************************
** Meta object code from reading C++ file 'chessengine.h'
**
** Created by: The Qt Meta Object Compiler version 67 (Qt 5.15.13)
**
** WARNING! All changes made in this file will be lost!
*****************************************************************************/

#include <memory>
#include "../../../projects/lib/src/chessengine.h"
#include <QtCore/qbytearray.h>
#include <QtCore/qmetatype.h>
#if !defined(Q_MOC_OUTPUT_REVISION)
#error "The header file 'chessengine.h' doesn't include <QObject>."
#elif Q_MOC_OUTPUT_REVISION != 67
#error "This file was generated using the moc from 5.15.13. It"
#error "cannot be used with the include files from this version of Qt."
#error "(The moc has changed too much.)"
#endif

QT_BEGIN_MOC_NAMESPACE
QT_WARNING_PUSH
QT_WARNING_DISABLE_DEPRECATED
struct qt_meta_stringdata_ChessEngine_t {
    QByteArrayData data[16];
    char stringdata0[178];
};
#define QT_MOC_LITERAL(idx, ofs, len) \
    Q_STATIC_BYTE_ARRAY_DATA_HEADER_INITIALIZER_WITH_OFFSET(len, \
    qptrdiff(offsetof(qt_meta_stringdata_ChessEngine_t, stringdata0) + ofs \
        - idx * sizeof(QByteArrayData)) \
    )
static const qt_meta_stringdata_ChessEngine_t qt_meta_stringdata_ChessEngine = {
    {
QT_MOC_LITERAL(0, 0, 11), // "ChessEngine"
QT_MOC_LITERAL(1, 12, 2), // "go"
QT_MOC_LITERAL(2, 15, 0), // ""
QT_MOC_LITERAL(3, 16, 4), // "quit"
QT_MOC_LITERAL(4, 21, 4), // "kill"
QT_MOC_LITERAL(5, 26, 9), // "onTimeout"
QT_MOC_LITERAL(6, 36, 11), // "onReadyRead"
QT_MOC_LITERAL(7, 48, 13), // "onPingTimeout"
QT_MOC_LITERAL(8, 62, 13), // "onIdleTimeout"
QT_MOC_LITERAL(9, 76, 4), // "pong"
QT_MOC_LITERAL(10, 81, 9), // "emitReady"
QT_MOC_LITERAL(11, 91, 15), // "onProtocolStart"
QT_MOC_LITERAL(12, 107, 16), // "flushWriteBuffer"
QT_MOC_LITERAL(13, 124, 16), // "clearWriteBuffer"
QT_MOC_LITERAL(14, 141, 13), // "onQuitTimeout"
QT_MOC_LITERAL(15, 155, 22) // "onProtocolStartTimeout"

    },
    "ChessEngine\0go\0\0quit\0kill\0onTimeout\0"
    "onReadyRead\0onPingTimeout\0onIdleTimeout\0"
    "pong\0emitReady\0onProtocolStart\0"
    "flushWriteBuffer\0clearWriteBuffer\0"
    "onQuitTimeout\0onProtocolStartTimeout"
};
#undef QT_MOC_LITERAL

static const uint qt_meta_data_ChessEngine[] = {

 // content:
       8,       // revision
       0,       // classname
       0,    0, // classinfo
      14,   14, // methods
       0,    0, // properties
       0,    0, // enums/sets
       0,    0, // constructors
       0,       // flags
       0,       // signalCount

 // slots: name, argc, parameters, tag, flags
       1,    0,   84,    2, 0x0a /* Public */,
       3,    0,   85,    2, 0x0a /* Public */,
       4,    0,   86,    2, 0x0a /* Public */,
       5,    0,   87,    2, 0x09 /* Protected */,
       6,    0,   88,    2, 0x09 /* Protected */,
       7,    0,   89,    2, 0x09 /* Protected */,
       8,    0,   90,    2, 0x09 /* Protected */,
       9,    1,   91,    2, 0x09 /* Protected */,
       9,    0,   94,    2, 0x29 /* Protected | MethodCloned */,
      11,    0,   95,    2, 0x09 /* Protected */,
      12,    0,   96,    2, 0x09 /* Protected */,
      13,    0,   97,    2, 0x09 /* Protected */,
      14,    0,   98,    2, 0x09 /* Protected */,
      15,    0,   99,    2, 0x09 /* Protected */,

 // slots: parameters
    QMetaType::Void,
    QMetaType::Void,
    QMetaType::Void,
    QMetaType::Void,
    QMetaType::Void,
    QMetaType::Void,
    QMetaType::Void,
    QMetaType::Void, QMetaType::Bool,   10,
    QMetaType::Void,
    QMetaType::Void,
    QMetaType::Void,
    QMetaType::Void,
    QMetaType::Void,
    QMetaType::Void,

       0        // eod
};

void ChessEngine::qt_static_metacall(QObject *_o, QMetaObject::Call _c, int _id, void **_a)
{
    if (_c == QMetaObject::InvokeMetaMethod) {
        auto *_t = static_cast<ChessEngine *>(_o);
        (void)_t;
        switch (_id) {
        case 0: _t->go(); break;
        case 1: _t->quit(); break;
        case 2: _t->kill(); break;
        case 3: _t->onTimeout(); break;
        case 4: _t->onReadyRead(); break;
        case 5: _t->onPingTimeout(); break;
        case 6: _t->onIdleTimeout(); break;
        case 7: _t->pong((*reinterpret_cast< bool(*)>(_a[1]))); break;
        case 8: _t->pong(); break;
        case 9: _t->onProtocolStart(); break;
        case 10: _t->flushWriteBuffer(); break;
        case 11: _t->clearWriteBuffer(); break;
        case 12: _t->onQuitTimeout(); break;
        case 13: _t->onProtocolStartTimeout(); break;
        default: ;
        }
    }
}

QT_INIT_METAOBJECT const QMetaObject ChessEngine::staticMetaObject = { {
    QMetaObject::SuperData::link<ChessPlayer::staticMetaObject>(),
    qt_meta_stringdata_ChessEngine.data,
    qt_meta_data_ChessEngine,
    qt_static_metacall,
    nullptr,
    nullptr
} };


const QMetaObject *ChessEngine::metaObject() const
{
    return QObject::d_ptr->metaObject ? QObject::d_ptr->dynamicMetaObject() : &staticMetaObject;
}

void *ChessEngine::qt_metacast(const char *_clname)
{
    if (!_clname) return nullptr;
    if (!strcmp(_clname, qt_meta_stringdata_ChessEngine.stringdata0))
        return static_cast<void*>(this);
    return ChessPlayer::qt_metacast(_clname);
}

int ChessEngine::qt_metacall(QMetaObject::Call _c, int _id, void **_a)
{
    _id = ChessPlayer::qt_metacall(_c, _id, _a);
    if (_id < 0)
        return _id;
    if (_c == QMetaObject::InvokeMetaMethod) {
        if (_id < 14)
            qt_static_metacall(this, _c, _id, _a);
        _id -= 14;
    } else if (_c == QMetaObject::RegisterMethodArgumentMetaType) {
        if (_id < 14)
            *reinterpret_cast<int*>(_a[0]) = -1;
        _id -= 14;
    }
    return _id;
}
QT_WARNING_POP
QT_END_MOC_NAMESPACE
