/****************************************************************************
** Meta object code from reading C++ file 'chessgame.h'
**
** Created by: The Qt Meta Object Compiler version 67 (Qt 5.15.13)
**
** WARNING! All changes made in this file will be lost!
*****************************************************************************/

#include <memory>
#include "../../../projects/lib/src/chessgame.h"
#include <QtCore/qbytearray.h>
#include <QtCore/qmetatype.h>
#if !defined(Q_MOC_OUTPUT_REVISION)
#error "The header file 'chessgame.h' doesn't include <QObject>."
#elif Q_MOC_OUTPUT_REVISION != 67
#error "This file was generated using the moc from 5.15.13. It"
#error "cannot be used with the include files from this version of Qt."
#error "(The moc has changed too much.)"
#endif

QT_BEGIN_MOC_NAMESPACE
QT_WARNING_PUSH
QT_WARNING_DISABLE_DEPRECATED
struct qt_meta_stringdata_ChessGame_t {
    QByteArrayData data[41];
    char stringdata0[414];
};
#define QT_MOC_LITERAL(idx, ofs, len) \
    Q_STATIC_BYTE_ARRAY_DATA_HEADER_INITIALIZER_WITH_OFFSET(len, \
    qptrdiff(offsetof(qt_meta_stringdata_ChessGame_t, stringdata0) + ofs \
        - idx * sizeof(QByteArrayData)) \
    )
static const qt_meta_stringdata_ChessGame_t qt_meta_stringdata_ChessGame = {
    {
QT_MOC_LITERAL(0, 0, 9), // "ChessGame"
QT_MOC_LITERAL(1, 10, 12), // "humanEnabled"
QT_MOC_LITERAL(2, 23, 0), // ""
QT_MOC_LITERAL(3, 24, 10), // "fenChanged"
QT_MOC_LITERAL(4, 35, 9), // "fenString"
QT_MOC_LITERAL(5, 45, 8), // "moveMade"
QT_MOC_LITERAL(6, 54, 18), // "Chess::GenericMove"
QT_MOC_LITERAL(7, 73, 4), // "move"
QT_MOC_LITERAL(8, 78, 9), // "sanString"
QT_MOC_LITERAL(9, 88, 7), // "comment"
QT_MOC_LITERAL(10, 96, 11), // "moveChanged"
QT_MOC_LITERAL(11, 108, 3), // "ply"
QT_MOC_LITERAL(12, 112, 12), // "scoreChanged"
QT_MOC_LITERAL(13, 125, 5), // "score"
QT_MOC_LITERAL(14, 131, 11), // "initialized"
QT_MOC_LITERAL(15, 143, 10), // "ChessGame*"
QT_MOC_LITERAL(16, 154, 4), // "game"
QT_MOC_LITERAL(17, 159, 7), // "started"
QT_MOC_LITERAL(18, 167, 8), // "finished"
QT_MOC_LITERAL(19, 176, 13), // "Chess::Result"
QT_MOC_LITERAL(20, 190, 6), // "result"
QT_MOC_LITERAL(21, 197, 11), // "startFailed"
QT_MOC_LITERAL(22, 209, 12), // "playersReady"
QT_MOC_LITERAL(23, 222, 5), // "start"
QT_MOC_LITERAL(24, 228, 5), // "pause"
QT_MOC_LITERAL(25, 234, 6), // "resume"
QT_MOC_LITERAL(26, 241, 4), // "stop"
QT_MOC_LITERAL(27, 246, 15), // "emitMoveChanged"
QT_MOC_LITERAL(28, 262, 4), // "kill"
QT_MOC_LITERAL(29, 267, 15), // "emitStartFailed"
QT_MOC_LITERAL(30, 283, 10), // "onMoveMade"
QT_MOC_LITERAL(31, 294, 11), // "Chess::Move"
QT_MOC_LITERAL(32, 306, 14), // "onAdjudication"
QT_MOC_LITERAL(33, 321, 13), // "onResignation"
QT_MOC_LITERAL(34, 335, 9), // "startGame"
QT_MOC_LITERAL(35, 345, 9), // "startTurn"
QT_MOC_LITERAL(36, 355, 6), // "finish"
QT_MOC_LITERAL(37, 362, 13), // "onResultClaim"
QT_MOC_LITERAL(38, 376, 13), // "onPlayerReady"
QT_MOC_LITERAL(39, 390, 11), // "syncPlayers"
QT_MOC_LITERAL(40, 402, 11) // "pauseThread"

    },
    "ChessGame\0humanEnabled\0\0fenChanged\0"
    "fenString\0moveMade\0Chess::GenericMove\0"
    "move\0sanString\0comment\0moveChanged\0"
    "ply\0scoreChanged\0score\0initialized\0"
    "ChessGame*\0game\0started\0finished\0"
    "Chess::Result\0result\0startFailed\0"
    "playersReady\0start\0pause\0resume\0stop\0"
    "emitMoveChanged\0kill\0emitStartFailed\0"
    "onMoveMade\0Chess::Move\0onAdjudication\0"
    "onResignation\0startGame\0startTurn\0"
    "finish\0onResultClaim\0onPlayerReady\0"
    "syncPlayers\0pauseThread"
};
#undef QT_MOC_LITERAL

static const uint qt_meta_data_ChessGame[] = {

 // content:
       8,       // revision
       0,       // classname
       0,    0, // classinfo
      32,   14, // methods
       0,    0, // properties
       0,    0, // enums/sets
       0,    0, // constructors
       0,       // flags
      15,       // signalCount

 // signals: name, argc, parameters, tag, flags
       1,    1,  174,    2, 0x06 /* Public */,
       3,    1,  177,    2, 0x06 /* Public */,
       5,    3,  180,    2, 0x06 /* Public */,
      10,    4,  187,    2, 0x06 /* Public */,
      12,    2,  196,    2, 0x06 /* Public */,
      14,    1,  201,    2, 0x06 /* Public */,
      14,    0,  204,    2, 0x26 /* Public | MethodCloned */,
      17,    1,  205,    2, 0x06 /* Public */,
      17,    0,  208,    2, 0x26 /* Public | MethodCloned */,
      18,    2,  209,    2, 0x06 /* Public */,
      18,    1,  214,    2, 0x26 /* Public | MethodCloned */,
      18,    0,  217,    2, 0x26 /* Public | MethodCloned */,
      21,    1,  218,    2, 0x06 /* Public */,
      21,    0,  221,    2, 0x26 /* Public | MethodCloned */,
      22,    0,  222,    2, 0x06 /* Public */,

 // slots: name, argc, parameters, tag, flags
      23,    0,  223,    2, 0x0a /* Public */,
      24,    0,  224,    2, 0x0a /* Public */,
      25,    0,  225,    2, 0x0a /* Public */,
      26,    1,  226,    2, 0x0a /* Public */,
      26,    0,  229,    2, 0x2a /* Public | MethodCloned */,
      28,    0,  230,    2, 0x0a /* Public */,
      29,    0,  231,    2, 0x0a /* Public */,
      30,    1,  232,    2, 0x0a /* Public */,
      32,    1,  235,    2, 0x0a /* Public */,
      33,    1,  238,    2, 0x0a /* Public */,
      34,    0,  241,    2, 0x08 /* Private */,
      35,    0,  242,    2, 0x08 /* Private */,
      36,    0,  243,    2, 0x08 /* Private */,
      37,    1,  244,    2, 0x08 /* Private */,
      38,    0,  247,    2, 0x08 /* Private */,
      39,    0,  248,    2, 0x08 /* Private */,
      40,    0,  249,    2, 0x08 /* Private */,

 // signals: parameters
    QMetaType::Void, QMetaType::Bool,    2,
    QMetaType::Void, QMetaType::QString,    4,
    QMetaType::Void, 0x80000000 | 6, QMetaType::QString, QMetaType::QString,    7,    8,    9,
    QMetaType::Void, QMetaType::Int, 0x80000000 | 6, QMetaType::QString, QMetaType::QString,   11,    7,    8,    9,
    QMetaType::Void, QMetaType::Int, QMetaType::Int,   11,   13,
    QMetaType::Void, 0x80000000 | 15,   16,
    QMetaType::Void,
    QMetaType::Void, 0x80000000 | 15,   16,
    QMetaType::Void,
    QMetaType::Void, 0x80000000 | 15, 0x80000000 | 19,   16,   20,
    QMetaType::Void, 0x80000000 | 15,   16,
    QMetaType::Void,
    QMetaType::Void, 0x80000000 | 15,   16,
    QMetaType::Void,
    QMetaType::Void,

 // slots: parameters
    QMetaType::Void,
    QMetaType::Void,
    QMetaType::Void,
    QMetaType::Void, QMetaType::Bool,   27,
    QMetaType::Void,
    QMetaType::Void,
    QMetaType::Void,
    QMetaType::Void, 0x80000000 | 31,    7,
    QMetaType::Void, 0x80000000 | 19,   20,
    QMetaType::Void, 0x80000000 | 19,   20,
    QMetaType::Void,
    QMetaType::Void,
    QMetaType::Void,
    QMetaType::Void, 0x80000000 | 19,   20,
    QMetaType::Void,
    QMetaType::Void,
    QMetaType::Void,

       0        // eod
};

void ChessGame::qt_static_metacall(QObject *_o, QMetaObject::Call _c, int _id, void **_a)
{
    if (_c == QMetaObject::InvokeMetaMethod) {
        auto *_t = static_cast<ChessGame *>(_o);
        (void)_t;
        switch (_id) {
        case 0: _t->humanEnabled((*reinterpret_cast< bool(*)>(_a[1]))); break;
        case 1: _t->fenChanged((*reinterpret_cast< const QString(*)>(_a[1]))); break;
        case 2: _t->moveMade((*reinterpret_cast< const Chess::GenericMove(*)>(_a[1])),(*reinterpret_cast< const QString(*)>(_a[2])),(*reinterpret_cast< const QString(*)>(_a[3]))); break;
        case 3: _t->moveChanged((*reinterpret_cast< int(*)>(_a[1])),(*reinterpret_cast< const Chess::GenericMove(*)>(_a[2])),(*reinterpret_cast< const QString(*)>(_a[3])),(*reinterpret_cast< const QString(*)>(_a[4]))); break;
        case 4: _t->scoreChanged((*reinterpret_cast< int(*)>(_a[1])),(*reinterpret_cast< int(*)>(_a[2]))); break;
        case 5: _t->initialized((*reinterpret_cast< ChessGame*(*)>(_a[1]))); break;
        case 6: _t->initialized(); break;
        case 7: _t->started((*reinterpret_cast< ChessGame*(*)>(_a[1]))); break;
        case 8: _t->started(); break;
        case 9: _t->finished((*reinterpret_cast< ChessGame*(*)>(_a[1])),(*reinterpret_cast< Chess::Result(*)>(_a[2]))); break;
        case 10: _t->finished((*reinterpret_cast< ChessGame*(*)>(_a[1]))); break;
        case 11: _t->finished(); break;
        case 12: _t->startFailed((*reinterpret_cast< ChessGame*(*)>(_a[1]))); break;
        case 13: _t->startFailed(); break;
        case 14: _t->playersReady(); break;
        case 15: _t->start(); break;
        case 16: _t->pause(); break;
        case 17: _t->resume(); break;
        case 18: _t->stop((*reinterpret_cast< bool(*)>(_a[1]))); break;
        case 19: _t->stop(); break;
        case 20: _t->kill(); break;
        case 21: _t->emitStartFailed(); break;
        case 22: _t->onMoveMade((*reinterpret_cast< const Chess::Move(*)>(_a[1]))); break;
        case 23: _t->onAdjudication((*reinterpret_cast< const Chess::Result(*)>(_a[1]))); break;
        case 24: _t->onResignation((*reinterpret_cast< const Chess::Result(*)>(_a[1]))); break;
        case 25: _t->startGame(); break;
        case 26: _t->startTurn(); break;
        case 27: _t->finish(); break;
        case 28: _t->onResultClaim((*reinterpret_cast< const Chess::Result(*)>(_a[1]))); break;
        case 29: _t->onPlayerReady(); break;
        case 30: _t->syncPlayers(); break;
        case 31: _t->pauseThread(); break;
        default: ;
        }
    } else if (_c == QMetaObject::RegisterMethodArgumentMetaType) {
        switch (_id) {
        default: *reinterpret_cast<int*>(_a[0]) = -1; break;
        case 2:
            switch (*reinterpret_cast<int*>(_a[1])) {
            default: *reinterpret_cast<int*>(_a[0]) = -1; break;
            case 0:
                *reinterpret_cast<int*>(_a[0]) = qRegisterMetaType< Chess::GenericMove >(); break;
            }
            break;
        case 3:
            switch (*reinterpret_cast<int*>(_a[1])) {
            default: *reinterpret_cast<int*>(_a[0]) = -1; break;
            case 1:
                *reinterpret_cast<int*>(_a[0]) = qRegisterMetaType< Chess::GenericMove >(); break;
            }
            break;
        case 5:
            switch (*reinterpret_cast<int*>(_a[1])) {
            default: *reinterpret_cast<int*>(_a[0]) = -1; break;
            case 0:
                *reinterpret_cast<int*>(_a[0]) = qRegisterMetaType< ChessGame* >(); break;
            }
            break;
        case 7:
            switch (*reinterpret_cast<int*>(_a[1])) {
            default: *reinterpret_cast<int*>(_a[0]) = -1; break;
            case 0:
                *reinterpret_cast<int*>(_a[0]) = qRegisterMetaType< ChessGame* >(); break;
            }
            break;
        case 9:
            switch (*reinterpret_cast<int*>(_a[1])) {
            default: *reinterpret_cast<int*>(_a[0]) = -1; break;
            case 1:
                *reinterpret_cast<int*>(_a[0]) = qRegisterMetaType< Chess::Result >(); break;
            case 0:
                *reinterpret_cast<int*>(_a[0]) = qRegisterMetaType< ChessGame* >(); break;
            }
            break;
        case 10:
            switch (*reinterpret_cast<int*>(_a[1])) {
            default: *reinterpret_cast<int*>(_a[0]) = -1; break;
            case 0:
                *reinterpret_cast<int*>(_a[0]) = qRegisterMetaType< ChessGame* >(); break;
            }
            break;
        case 12:
            switch (*reinterpret_cast<int*>(_a[1])) {
            default: *reinterpret_cast<int*>(_a[0]) = -1; break;
            case 0:
                *reinterpret_cast<int*>(_a[0]) = qRegisterMetaType< ChessGame* >(); break;
            }
            break;
        case 22:
            switch (*reinterpret_cast<int*>(_a[1])) {
            default: *reinterpret_cast<int*>(_a[0]) = -1; break;
            case 0:
                *reinterpret_cast<int*>(_a[0]) = qRegisterMetaType< Chess::Move >(); break;
            }
            break;
        case 23:
            switch (*reinterpret_cast<int*>(_a[1])) {
            default: *reinterpret_cast<int*>(_a[0]) = -1; break;
            case 0:
                *reinterpret_cast<int*>(_a[0]) = qRegisterMetaType< Chess::Result >(); break;
            }
            break;
        case 24:
            switch (*reinterpret_cast<int*>(_a[1])) {
            default: *reinterpret_cast<int*>(_a[0]) = -1; break;
            case 0:
                *reinterpret_cast<int*>(_a[0]) = qRegisterMetaType< Chess::Result >(); break;
            }
            break;
        case 28:
            switch (*reinterpret_cast<int*>(_a[1])) {
            default: *reinterpret_cast<int*>(_a[0]) = -1; break;
            case 0:
                *reinterpret_cast<int*>(_a[0]) = qRegisterMetaType< Chess::Result >(); break;
            }
            break;
        }
    } else if (_c == QMetaObject::IndexOfMethod) {
        int *result = reinterpret_cast<int *>(_a[0]);
        {
            using _t = void (ChessGame::*)(bool );
            if (*reinterpret_cast<_t *>(_a[1]) == static_cast<_t>(&ChessGame::humanEnabled)) {
                *result = 0;
                return;
            }
        }
        {
            using _t = void (ChessGame::*)(const QString & );
            if (*reinterpret_cast<_t *>(_a[1]) == static_cast<_t>(&ChessGame::fenChanged)) {
                *result = 1;
                return;
            }
        }
        {
            using _t = void (ChessGame::*)(const Chess::GenericMove & , const QString & , const QString & );
            if (*reinterpret_cast<_t *>(_a[1]) == static_cast<_t>(&ChessGame::moveMade)) {
                *result = 2;
                return;
            }
        }
        {
            using _t = void (ChessGame::*)(int , const Chess::GenericMove & , const QString & , const QString & );
            if (*reinterpret_cast<_t *>(_a[1]) == static_cast<_t>(&ChessGame::moveChanged)) {
                *result = 3;
                return;
            }
        }
        {
            using _t = void (ChessGame::*)(int , int );
            if (*reinterpret_cast<_t *>(_a[1]) == static_cast<_t>(&ChessGame::scoreChanged)) {
                *result = 4;
                return;
            }
        }
        {
            using _t = void (ChessGame::*)(ChessGame * );
            if (*reinterpret_cast<_t *>(_a[1]) == static_cast<_t>(&ChessGame::initialized)) {
                *result = 5;
                return;
            }
        }
        {
            using _t = void (ChessGame::*)(ChessGame * );
            if (*reinterpret_cast<_t *>(_a[1]) == static_cast<_t>(&ChessGame::started)) {
                *result = 7;
                return;
            }
        }
        {
            using _t = void (ChessGame::*)(ChessGame * , Chess::Result );
            if (*reinterpret_cast<_t *>(_a[1]) == static_cast<_t>(&ChessGame::finished)) {
                *result = 9;
                return;
            }
        }
        {
            using _t = void (ChessGame::*)(ChessGame * );
            if (*reinterpret_cast<_t *>(_a[1]) == static_cast<_t>(&ChessGame::startFailed)) {
                *result = 12;
                return;
            }
        }
        {
            using _t = void (ChessGame::*)();
            if (*reinterpret_cast<_t *>(_a[1]) == static_cast<_t>(&ChessGame::playersReady)) {
                *result = 14;
                return;
            }
        }
    }
}

QT_INIT_METAOBJECT const QMetaObject ChessGame::staticMetaObject = { {
    QMetaObject::SuperData::link<QObject::staticMetaObject>(),
    qt_meta_stringdata_ChessGame.data,
    qt_meta_data_ChessGame,
    qt_static_metacall,
    nullptr,
    nullptr
} };


const QMetaObject *ChessGame::metaObject() const
{
    return QObject::d_ptr->metaObject ? QObject::d_ptr->dynamicMetaObject() : &staticMetaObject;
}

void *ChessGame::qt_metacast(const char *_clname)
{
    if (!_clname) return nullptr;
    if (!strcmp(_clname, qt_meta_stringdata_ChessGame.stringdata0))
        return static_cast<void*>(this);
    return QObject::qt_metacast(_clname);
}

int ChessGame::qt_metacall(QMetaObject::Call _c, int _id, void **_a)
{
    _id = QObject::qt_metacall(_c, _id, _a);
    if (_id < 0)
        return _id;
    if (_c == QMetaObject::InvokeMetaMethod) {
        if (_id < 32)
            qt_static_metacall(this, _c, _id, _a);
        _id -= 32;
    } else if (_c == QMetaObject::RegisterMethodArgumentMetaType) {
        if (_id < 32)
            qt_static_metacall(this, _c, _id, _a);
        _id -= 32;
    }
    return _id;
}

// SIGNAL 0
void ChessGame::humanEnabled(bool _t1)
{
    void *_a[] = { nullptr, const_cast<void*>(reinterpret_cast<const void*>(std::addressof(_t1))) };
    QMetaObject::activate(this, &staticMetaObject, 0, _a);
}

// SIGNAL 1
void ChessGame::fenChanged(const QString & _t1)
{
    void *_a[] = { nullptr, const_cast<void*>(reinterpret_cast<const void*>(std::addressof(_t1))) };
    QMetaObject::activate(this, &staticMetaObject, 1, _a);
}

// SIGNAL 2
void ChessGame::moveMade(const Chess::GenericMove & _t1, const QString & _t2, const QString & _t3)
{
    void *_a[] = { nullptr, const_cast<void*>(reinterpret_cast<const void*>(std::addressof(_t1))), const_cast<void*>(reinterpret_cast<const void*>(std::addressof(_t2))), const_cast<void*>(reinterpret_cast<const void*>(std::addressof(_t3))) };
    QMetaObject::activate(this, &staticMetaObject, 2, _a);
}

// SIGNAL 3
void ChessGame::moveChanged(int _t1, const Chess::GenericMove & _t2, const QString & _t3, const QString & _t4)
{
    void *_a[] = { nullptr, const_cast<void*>(reinterpret_cast<const void*>(std::addressof(_t1))), const_cast<void*>(reinterpret_cast<const void*>(std::addressof(_t2))), const_cast<void*>(reinterpret_cast<const void*>(std::addressof(_t3))), const_cast<void*>(reinterpret_cast<const void*>(std::addressof(_t4))) };
    QMetaObject::activate(this, &staticMetaObject, 3, _a);
}

// SIGNAL 4
void ChessGame::scoreChanged(int _t1, int _t2)
{
    void *_a[] = { nullptr, const_cast<void*>(reinterpret_cast<const void*>(std::addressof(_t1))), const_cast<void*>(reinterpret_cast<const void*>(std::addressof(_t2))) };
    QMetaObject::activate(this, &staticMetaObject, 4, _a);
}

// SIGNAL 5
void ChessGame::initialized(ChessGame * _t1)
{
    void *_a[] = { nullptr, const_cast<void*>(reinterpret_cast<const void*>(std::addressof(_t1))) };
    QMetaObject::activate(this, &staticMetaObject, 5, _a);
}

// SIGNAL 7
void ChessGame::started(ChessGame * _t1)
{
    void *_a[] = { nullptr, const_cast<void*>(reinterpret_cast<const void*>(std::addressof(_t1))) };
    QMetaObject::activate(this, &staticMetaObject, 7, _a);
}

// SIGNAL 9
void ChessGame::finished(ChessGame * _t1, Chess::Result _t2)
{
    void *_a[] = { nullptr, const_cast<void*>(reinterpret_cast<const void*>(std::addressof(_t1))), const_cast<void*>(reinterpret_cast<const void*>(std::addressof(_t2))) };
    QMetaObject::activate(this, &staticMetaObject, 9, _a);
}

// SIGNAL 12
void ChessGame::startFailed(ChessGame * _t1)
{
    void *_a[] = { nullptr, const_cast<void*>(reinterpret_cast<const void*>(std::addressof(_t1))) };
    QMetaObject::activate(this, &staticMetaObject, 12, _a);
}

// SIGNAL 14
void ChessGame::playersReady()
{
    QMetaObject::activate(this, &staticMetaObject, 14, nullptr);
}
QT_WARNING_POP
QT_END_MOC_NAMESPACE
