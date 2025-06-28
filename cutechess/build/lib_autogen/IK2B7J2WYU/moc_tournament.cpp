/****************************************************************************
** Meta object code from reading C++ file 'tournament.h'
**
** Created by: The Qt Meta Object Compiler version 67 (Qt 5.15.13)
**
** WARNING! All changes made in this file will be lost!
*****************************************************************************/

#include <memory>
#include "../../../projects/lib/src/tournament.h"
#include <QtCore/qbytearray.h>
#include <QtCore/qmetatype.h>
#if !defined(Q_MOC_OUTPUT_REVISION)
#error "The header file 'tournament.h' doesn't include <QObject>."
#elif Q_MOC_OUTPUT_REVISION != 67
#error "This file was generated using the moc from 5.15.13. It"
#error "cannot be used with the include files from this version of Qt."
#error "(The moc has changed too much.)"
#endif

QT_BEGIN_MOC_NAMESPACE
QT_WARNING_PUSH
QT_WARNING_DISABLE_DEPRECATED
struct qt_meta_stringdata_Tournament_t {
    QByteArrayData data[26];
    char stringdata0[263];
};
#define QT_MOC_LITERAL(idx, ofs, len) \
    Q_STATIC_BYTE_ARRAY_DATA_HEADER_INITIALIZER_WITH_OFFSET(len, \
    qptrdiff(offsetof(qt_meta_stringdata_Tournament_t, stringdata0) + ofs \
        - idx * sizeof(QByteArrayData)) \
    )
static const qt_meta_stringdata_Tournament_t qt_meta_stringdata_Tournament = {
    {
QT_MOC_LITERAL(0, 0, 10), // "Tournament"
QT_MOC_LITERAL(1, 11, 11), // "gameStarted"
QT_MOC_LITERAL(2, 23, 0), // ""
QT_MOC_LITERAL(3, 24, 10), // "ChessGame*"
QT_MOC_LITERAL(4, 35, 4), // "game"
QT_MOC_LITERAL(5, 40, 6), // "number"
QT_MOC_LITERAL(6, 47, 10), // "whiteIndex"
QT_MOC_LITERAL(7, 58, 10), // "blackIndex"
QT_MOC_LITERAL(8, 69, 12), // "gameFinished"
QT_MOC_LITERAL(9, 82, 14), // "gameTerminated"
QT_MOC_LITERAL(10, 97, 5), // "index"
QT_MOC_LITERAL(11, 103, 13), // "Chess::Result"
QT_MOC_LITERAL(12, 117, 6), // "result"
QT_MOC_LITERAL(13, 124, 8), // "finished"
QT_MOC_LITERAL(14, 133, 5), // "start"
QT_MOC_LITERAL(15, 139, 4), // "stop"
QT_MOC_LITERAL(16, 144, 13), // "startNextGame"
QT_MOC_LITERAL(17, 158, 8), // "writePgn"
QT_MOC_LITERAL(18, 167, 8), // "PgnGame*"
QT_MOC_LITERAL(19, 176, 3), // "pgn"
QT_MOC_LITERAL(20, 180, 10), // "gameNumber"
QT_MOC_LITERAL(21, 191, 8), // "writeEpd"
QT_MOC_LITERAL(22, 200, 13), // "onGameStarted"
QT_MOC_LITERAL(23, 214, 14), // "onGameFinished"
QT_MOC_LITERAL(24, 229, 15), // "onGameDestroyed"
QT_MOC_LITERAL(25, 245, 17) // "onGameStartFailed"

    },
    "Tournament\0gameStarted\0\0ChessGame*\0"
    "game\0number\0whiteIndex\0blackIndex\0"
    "gameFinished\0gameTerminated\0index\0"
    "Chess::Result\0result\0finished\0start\0"
    "stop\0startNextGame\0writePgn\0PgnGame*\0"
    "pgn\0gameNumber\0writeEpd\0onGameStarted\0"
    "onGameFinished\0onGameDestroyed\0"
    "onGameStartFailed"
};
#undef QT_MOC_LITERAL

static const uint qt_meta_data_Tournament[] = {

 // content:
       8,       // revision
       0,       // classname
       0,    0, // classinfo
      13,   14, // methods
       0,    0, // properties
       0,    0, // enums/sets
       0,    0, // constructors
       0,       // flags
       4,       // signalCount

 // signals: name, argc, parameters, tag, flags
       1,    4,   79,    2, 0x06 /* Public */,
       8,    4,   88,    2, 0x06 /* Public */,
       9,    2,   97,    2, 0x06 /* Public */,
      13,    0,  102,    2, 0x06 /* Public */,

 // slots: name, argc, parameters, tag, flags
      14,    0,  103,    2, 0x0a /* Public */,
      15,    0,  104,    2, 0x0a /* Public */,
      16,    0,  105,    2, 0x08 /* Private */,
      17,    2,  106,    2, 0x08 /* Private */,
      21,    1,  111,    2, 0x08 /* Private */,
      22,    1,  114,    2, 0x08 /* Private */,
      23,    1,  117,    2, 0x08 /* Private */,
      24,    1,  120,    2, 0x08 /* Private */,
      25,    1,  123,    2, 0x08 /* Private */,

 // signals: parameters
    QMetaType::Void, 0x80000000 | 3, QMetaType::Int, QMetaType::Int, QMetaType::Int,    4,    5,    6,    7,
    QMetaType::Void, 0x80000000 | 3, QMetaType::Int, QMetaType::Int, QMetaType::Int,    4,    5,    6,    7,
    QMetaType::Void, QMetaType::Int, 0x80000000 | 11,   10,   12,
    QMetaType::Void,

 // slots: parameters
    QMetaType::Void,
    QMetaType::Void,
    QMetaType::Void,
    QMetaType::Bool, 0x80000000 | 18, QMetaType::Int,   19,   20,
    QMetaType::Bool, 0x80000000 | 3,    4,
    QMetaType::Void, 0x80000000 | 3,    4,
    QMetaType::Void, 0x80000000 | 3,    4,
    QMetaType::Void, 0x80000000 | 3,    4,
    QMetaType::Void, 0x80000000 | 3,    4,

       0        // eod
};

void Tournament::qt_static_metacall(QObject *_o, QMetaObject::Call _c, int _id, void **_a)
{
    if (_c == QMetaObject::InvokeMetaMethod) {
        auto *_t = static_cast<Tournament *>(_o);
        (void)_t;
        switch (_id) {
        case 0: _t->gameStarted((*reinterpret_cast< ChessGame*(*)>(_a[1])),(*reinterpret_cast< int(*)>(_a[2])),(*reinterpret_cast< int(*)>(_a[3])),(*reinterpret_cast< int(*)>(_a[4]))); break;
        case 1: _t->gameFinished((*reinterpret_cast< ChessGame*(*)>(_a[1])),(*reinterpret_cast< int(*)>(_a[2])),(*reinterpret_cast< int(*)>(_a[3])),(*reinterpret_cast< int(*)>(_a[4]))); break;
        case 2: _t->gameTerminated((*reinterpret_cast< int(*)>(_a[1])),(*reinterpret_cast< Chess::Result(*)>(_a[2]))); break;
        case 3: _t->finished(); break;
        case 4: _t->start(); break;
        case 5: _t->stop(); break;
        case 6: _t->startNextGame(); break;
        case 7: { bool _r = _t->writePgn((*reinterpret_cast< PgnGame*(*)>(_a[1])),(*reinterpret_cast< int(*)>(_a[2])));
            if (_a[0]) *reinterpret_cast< bool*>(_a[0]) = std::move(_r); }  break;
        case 8: { bool _r = _t->writeEpd((*reinterpret_cast< ChessGame*(*)>(_a[1])));
            if (_a[0]) *reinterpret_cast< bool*>(_a[0]) = std::move(_r); }  break;
        case 9: _t->onGameStarted((*reinterpret_cast< ChessGame*(*)>(_a[1]))); break;
        case 10: _t->onGameFinished((*reinterpret_cast< ChessGame*(*)>(_a[1]))); break;
        case 11: _t->onGameDestroyed((*reinterpret_cast< ChessGame*(*)>(_a[1]))); break;
        case 12: _t->onGameStartFailed((*reinterpret_cast< ChessGame*(*)>(_a[1]))); break;
        default: ;
        }
    } else if (_c == QMetaObject::RegisterMethodArgumentMetaType) {
        switch (_id) {
        default: *reinterpret_cast<int*>(_a[0]) = -1; break;
        case 2:
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
            using _t = void (Tournament::*)(ChessGame * , int , int , int );
            if (*reinterpret_cast<_t *>(_a[1]) == static_cast<_t>(&Tournament::gameStarted)) {
                *result = 0;
                return;
            }
        }
        {
            using _t = void (Tournament::*)(ChessGame * , int , int , int );
            if (*reinterpret_cast<_t *>(_a[1]) == static_cast<_t>(&Tournament::gameFinished)) {
                *result = 1;
                return;
            }
        }
        {
            using _t = void (Tournament::*)(int , Chess::Result );
            if (*reinterpret_cast<_t *>(_a[1]) == static_cast<_t>(&Tournament::gameTerminated)) {
                *result = 2;
                return;
            }
        }
        {
            using _t = void (Tournament::*)();
            if (*reinterpret_cast<_t *>(_a[1]) == static_cast<_t>(&Tournament::finished)) {
                *result = 3;
                return;
            }
        }
    }
}

QT_INIT_METAOBJECT const QMetaObject Tournament::staticMetaObject = { {
    QMetaObject::SuperData::link<QObject::staticMetaObject>(),
    qt_meta_stringdata_Tournament.data,
    qt_meta_data_Tournament,
    qt_static_metacall,
    nullptr,
    nullptr
} };


const QMetaObject *Tournament::metaObject() const
{
    return QObject::d_ptr->metaObject ? QObject::d_ptr->dynamicMetaObject() : &staticMetaObject;
}

void *Tournament::qt_metacast(const char *_clname)
{
    if (!_clname) return nullptr;
    if (!strcmp(_clname, qt_meta_stringdata_Tournament.stringdata0))
        return static_cast<void*>(this);
    return QObject::qt_metacast(_clname);
}

int Tournament::qt_metacall(QMetaObject::Call _c, int _id, void **_a)
{
    _id = QObject::qt_metacall(_c, _id, _a);
    if (_id < 0)
        return _id;
    if (_c == QMetaObject::InvokeMetaMethod) {
        if (_id < 13)
            qt_static_metacall(this, _c, _id, _a);
        _id -= 13;
    } else if (_c == QMetaObject::RegisterMethodArgumentMetaType) {
        if (_id < 13)
            qt_static_metacall(this, _c, _id, _a);
        _id -= 13;
    }
    return _id;
}

// SIGNAL 0
void Tournament::gameStarted(ChessGame * _t1, int _t2, int _t3, int _t4)
{
    void *_a[] = { nullptr, const_cast<void*>(reinterpret_cast<const void*>(std::addressof(_t1))), const_cast<void*>(reinterpret_cast<const void*>(std::addressof(_t2))), const_cast<void*>(reinterpret_cast<const void*>(std::addressof(_t3))), const_cast<void*>(reinterpret_cast<const void*>(std::addressof(_t4))) };
    QMetaObject::activate(this, &staticMetaObject, 0, _a);
}

// SIGNAL 1
void Tournament::gameFinished(ChessGame * _t1, int _t2, int _t3, int _t4)
{
    void *_a[] = { nullptr, const_cast<void*>(reinterpret_cast<const void*>(std::addressof(_t1))), const_cast<void*>(reinterpret_cast<const void*>(std::addressof(_t2))), const_cast<void*>(reinterpret_cast<const void*>(std::addressof(_t3))), const_cast<void*>(reinterpret_cast<const void*>(std::addressof(_t4))) };
    QMetaObject::activate(this, &staticMetaObject, 1, _a);
}

// SIGNAL 2
void Tournament::gameTerminated(int _t1, Chess::Result _t2)
{
    void *_a[] = { nullptr, const_cast<void*>(reinterpret_cast<const void*>(std::addressof(_t1))), const_cast<void*>(reinterpret_cast<const void*>(std::addressof(_t2))) };
    QMetaObject::activate(this, &staticMetaObject, 2, _a);
}

// SIGNAL 3
void Tournament::finished()
{
    QMetaObject::activate(this, &staticMetaObject, 3, nullptr);
}
struct qt_meta_stringdata_ResultFormatter_t {
    QByteArrayData data[1];
    char stringdata0[16];
};
#define QT_MOC_LITERAL(idx, ofs, len) \
    Q_STATIC_BYTE_ARRAY_DATA_HEADER_INITIALIZER_WITH_OFFSET(len, \
    qptrdiff(offsetof(qt_meta_stringdata_ResultFormatter_t, stringdata0) + ofs \
        - idx * sizeof(QByteArrayData)) \
    )
static const qt_meta_stringdata_ResultFormatter_t qt_meta_stringdata_ResultFormatter = {
    {
QT_MOC_LITERAL(0, 0, 15) // "ResultFormatter"

    },
    "ResultFormatter"
};
#undef QT_MOC_LITERAL

static const uint qt_meta_data_ResultFormatter[] = {

 // content:
       8,       // revision
       0,       // classname
       0,    0, // classinfo
       0,    0, // methods
       0,    0, // properties
       0,    0, // enums/sets
       0,    0, // constructors
       0,       // flags
       0,       // signalCount

       0        // eod
};

void ResultFormatter::qt_static_metacall(QObject *_o, QMetaObject::Call _c, int _id, void **_a)
{
    (void)_o;
    (void)_id;
    (void)_c;
    (void)_a;
}

QT_INIT_METAOBJECT const QMetaObject ResultFormatter::staticMetaObject = { {
    QMetaObject::SuperData::link<QObject::staticMetaObject>(),
    qt_meta_stringdata_ResultFormatter.data,
    qt_meta_data_ResultFormatter,
    qt_static_metacall,
    nullptr,
    nullptr
} };


const QMetaObject *ResultFormatter::metaObject() const
{
    return QObject::d_ptr->metaObject ? QObject::d_ptr->dynamicMetaObject() : &staticMetaObject;
}

void *ResultFormatter::qt_metacast(const char *_clname)
{
    if (!_clname) return nullptr;
    if (!strcmp(_clname, qt_meta_stringdata_ResultFormatter.stringdata0))
        return static_cast<void*>(this);
    return QObject::qt_metacast(_clname);
}

int ResultFormatter::qt_metacall(QMetaObject::Call _c, int _id, void **_a)
{
    _id = QObject::qt_metacall(_c, _id, _a);
    return _id;
}
QT_WARNING_POP
QT_END_MOC_NAMESPACE
