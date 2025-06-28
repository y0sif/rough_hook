/****************************************************************************
** Meta object code from reading C++ file 'chessplayer.h'
**
** Created by: The Qt Meta Object Compiler version 67 (Qt 5.15.13)
**
** WARNING! All changes made in this file will be lost!
*****************************************************************************/

#include <memory>
#include "../../../projects/lib/src/chessplayer.h"
#include <QtCore/qbytearray.h>
#include <QtCore/qmetatype.h>
#if !defined(Q_MOC_OUTPUT_REVISION)
#error "The header file 'chessplayer.h' doesn't include <QObject>."
#elif Q_MOC_OUTPUT_REVISION != 67
#error "This file was generated using the moc from 5.15.13. It"
#error "cannot be used with the include files from this version of Qt."
#error "(The moc has changed too much.)"
#endif

QT_BEGIN_MOC_NAMESPACE
QT_WARNING_PUSH
QT_WARNING_DISABLE_DEPRECATED
struct qt_meta_stringdata_ChessPlayer_t {
    QByteArrayData data[25];
    char stringdata0[229];
};
#define QT_MOC_LITERAL(idx, ofs, len) \
    Q_STATIC_BYTE_ARRAY_DATA_HEADER_INITIALIZER_WITH_OFFSET(len, \
    qptrdiff(offsetof(qt_meta_stringdata_ChessPlayer_t, stringdata0) + ofs \
        - idx * sizeof(QByteArrayData)) \
    )
static const qt_meta_stringdata_ChessPlayer_t qt_meta_stringdata_ChessPlayer = {
    {
QT_MOC_LITERAL(0, 0, 11), // "ChessPlayer"
QT_MOC_LITERAL(1, 12, 12), // "disconnected"
QT_MOC_LITERAL(2, 25, 0), // ""
QT_MOC_LITERAL(3, 26, 5), // "ready"
QT_MOC_LITERAL(4, 32, 15), // "startedThinking"
QT_MOC_LITERAL(5, 48, 8), // "timeLeft"
QT_MOC_LITERAL(6, 57, 15), // "stoppedThinking"
QT_MOC_LITERAL(7, 73, 8), // "thinking"
QT_MOC_LITERAL(8, 82, 14), // "MoveEvaluation"
QT_MOC_LITERAL(9, 97, 4), // "eval"
QT_MOC_LITERAL(10, 102, 8), // "moveMade"
QT_MOC_LITERAL(11, 111, 11), // "Chess::Move"
QT_MOC_LITERAL(12, 123, 4), // "move"
QT_MOC_LITERAL(13, 128, 11), // "resultClaim"
QT_MOC_LITERAL(14, 140, 13), // "Chess::Result"
QT_MOC_LITERAL(15, 154, 6), // "result"
QT_MOC_LITERAL(16, 161, 12), // "debugMessage"
QT_MOC_LITERAL(17, 174, 4), // "data"
QT_MOC_LITERAL(18, 179, 11), // "nameChanged"
QT_MOC_LITERAL(19, 191, 4), // "name"
QT_MOC_LITERAL(20, 196, 2), // "go"
QT_MOC_LITERAL(21, 199, 4), // "quit"
QT_MOC_LITERAL(22, 204, 4), // "kill"
QT_MOC_LITERAL(23, 209, 9), // "onCrashed"
QT_MOC_LITERAL(24, 219, 9) // "onTimeout"

    },
    "ChessPlayer\0disconnected\0\0ready\0"
    "startedThinking\0timeLeft\0stoppedThinking\0"
    "thinking\0MoveEvaluation\0eval\0moveMade\0"
    "Chess::Move\0move\0resultClaim\0Chess::Result\0"
    "result\0debugMessage\0data\0nameChanged\0"
    "name\0go\0quit\0kill\0onCrashed\0onTimeout"
};
#undef QT_MOC_LITERAL

static const uint qt_meta_data_ChessPlayer[] = {

 // content:
       8,       // revision
       0,       // classname
       0,    0, // classinfo
      14,   14, // methods
       0,    0, // properties
       0,    0, // enums/sets
       0,    0, // constructors
       0,       // flags
       9,       // signalCount

 // signals: name, argc, parameters, tag, flags
       1,    0,   84,    2, 0x06 /* Public */,
       3,    0,   85,    2, 0x06 /* Public */,
       4,    1,   86,    2, 0x06 /* Public */,
       6,    0,   89,    2, 0x06 /* Public */,
       7,    1,   90,    2, 0x06 /* Public */,
      10,    1,   93,    2, 0x06 /* Public */,
      13,    1,   96,    2, 0x06 /* Public */,
      16,    1,   99,    2, 0x06 /* Public */,
      18,    1,  102,    2, 0x06 /* Public */,

 // slots: name, argc, parameters, tag, flags
      20,    0,  105,    2, 0x0a /* Public */,
      21,    0,  106,    2, 0x0a /* Public */,
      22,    0,  107,    2, 0x0a /* Public */,
      23,    0,  108,    2, 0x09 /* Protected */,
      24,    0,  109,    2, 0x09 /* Protected */,

 // signals: parameters
    QMetaType::Void,
    QMetaType::Void,
    QMetaType::Void, QMetaType::Int,    5,
    QMetaType::Void,
    QMetaType::Void, 0x80000000 | 8,    9,
    QMetaType::Void, 0x80000000 | 11,   12,
    QMetaType::Void, 0x80000000 | 14,   15,
    QMetaType::Void, QMetaType::QString,   17,
    QMetaType::Void, QMetaType::QString,   19,

 // slots: parameters
    QMetaType::Void,
    QMetaType::Void,
    QMetaType::Void,
    QMetaType::Void,
    QMetaType::Void,

       0        // eod
};

void ChessPlayer::qt_static_metacall(QObject *_o, QMetaObject::Call _c, int _id, void **_a)
{
    if (_c == QMetaObject::InvokeMetaMethod) {
        auto *_t = static_cast<ChessPlayer *>(_o);
        (void)_t;
        switch (_id) {
        case 0: _t->disconnected(); break;
        case 1: _t->ready(); break;
        case 2: _t->startedThinking((*reinterpret_cast< int(*)>(_a[1]))); break;
        case 3: _t->stoppedThinking(); break;
        case 4: _t->thinking((*reinterpret_cast< const MoveEvaluation(*)>(_a[1]))); break;
        case 5: _t->moveMade((*reinterpret_cast< const Chess::Move(*)>(_a[1]))); break;
        case 6: _t->resultClaim((*reinterpret_cast< const Chess::Result(*)>(_a[1]))); break;
        case 7: _t->debugMessage((*reinterpret_cast< const QString(*)>(_a[1]))); break;
        case 8: _t->nameChanged((*reinterpret_cast< const QString(*)>(_a[1]))); break;
        case 9: _t->go(); break;
        case 10: _t->quit(); break;
        case 11: _t->kill(); break;
        case 12: _t->onCrashed(); break;
        case 13: _t->onTimeout(); break;
        default: ;
        }
    } else if (_c == QMetaObject::RegisterMethodArgumentMetaType) {
        switch (_id) {
        default: *reinterpret_cast<int*>(_a[0]) = -1; break;
        case 4:
            switch (*reinterpret_cast<int*>(_a[1])) {
            default: *reinterpret_cast<int*>(_a[0]) = -1; break;
            case 0:
                *reinterpret_cast<int*>(_a[0]) = qRegisterMetaType< MoveEvaluation >(); break;
            }
            break;
        case 5:
            switch (*reinterpret_cast<int*>(_a[1])) {
            default: *reinterpret_cast<int*>(_a[0]) = -1; break;
            case 0:
                *reinterpret_cast<int*>(_a[0]) = qRegisterMetaType< Chess::Move >(); break;
            }
            break;
        case 6:
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
            using _t = void (ChessPlayer::*)();
            if (*reinterpret_cast<_t *>(_a[1]) == static_cast<_t>(&ChessPlayer::disconnected)) {
                *result = 0;
                return;
            }
        }
        {
            using _t = void (ChessPlayer::*)();
            if (*reinterpret_cast<_t *>(_a[1]) == static_cast<_t>(&ChessPlayer::ready)) {
                *result = 1;
                return;
            }
        }
        {
            using _t = void (ChessPlayer::*)(int );
            if (*reinterpret_cast<_t *>(_a[1]) == static_cast<_t>(&ChessPlayer::startedThinking)) {
                *result = 2;
                return;
            }
        }
        {
            using _t = void (ChessPlayer::*)();
            if (*reinterpret_cast<_t *>(_a[1]) == static_cast<_t>(&ChessPlayer::stoppedThinking)) {
                *result = 3;
                return;
            }
        }
        {
            using _t = void (ChessPlayer::*)(const MoveEvaluation & );
            if (*reinterpret_cast<_t *>(_a[1]) == static_cast<_t>(&ChessPlayer::thinking)) {
                *result = 4;
                return;
            }
        }
        {
            using _t = void (ChessPlayer::*)(const Chess::Move & );
            if (*reinterpret_cast<_t *>(_a[1]) == static_cast<_t>(&ChessPlayer::moveMade)) {
                *result = 5;
                return;
            }
        }
        {
            using _t = void (ChessPlayer::*)(const Chess::Result & );
            if (*reinterpret_cast<_t *>(_a[1]) == static_cast<_t>(&ChessPlayer::resultClaim)) {
                *result = 6;
                return;
            }
        }
        {
            using _t = void (ChessPlayer::*)(const QString & );
            if (*reinterpret_cast<_t *>(_a[1]) == static_cast<_t>(&ChessPlayer::debugMessage)) {
                *result = 7;
                return;
            }
        }
        {
            using _t = void (ChessPlayer::*)(const QString & );
            if (*reinterpret_cast<_t *>(_a[1]) == static_cast<_t>(&ChessPlayer::nameChanged)) {
                *result = 8;
                return;
            }
        }
    }
}

QT_INIT_METAOBJECT const QMetaObject ChessPlayer::staticMetaObject = { {
    QMetaObject::SuperData::link<QObject::staticMetaObject>(),
    qt_meta_stringdata_ChessPlayer.data,
    qt_meta_data_ChessPlayer,
    qt_static_metacall,
    nullptr,
    nullptr
} };


const QMetaObject *ChessPlayer::metaObject() const
{
    return QObject::d_ptr->metaObject ? QObject::d_ptr->dynamicMetaObject() : &staticMetaObject;
}

void *ChessPlayer::qt_metacast(const char *_clname)
{
    if (!_clname) return nullptr;
    if (!strcmp(_clname, qt_meta_stringdata_ChessPlayer.stringdata0))
        return static_cast<void*>(this);
    return QObject::qt_metacast(_clname);
}

int ChessPlayer::qt_metacall(QMetaObject::Call _c, int _id, void **_a)
{
    _id = QObject::qt_metacall(_c, _id, _a);
    if (_id < 0)
        return _id;
    if (_c == QMetaObject::InvokeMetaMethod) {
        if (_id < 14)
            qt_static_metacall(this, _c, _id, _a);
        _id -= 14;
    } else if (_c == QMetaObject::RegisterMethodArgumentMetaType) {
        if (_id < 14)
            qt_static_metacall(this, _c, _id, _a);
        _id -= 14;
    }
    return _id;
}

// SIGNAL 0
void ChessPlayer::disconnected()
{
    QMetaObject::activate(this, &staticMetaObject, 0, nullptr);
}

// SIGNAL 1
void ChessPlayer::ready()
{
    QMetaObject::activate(this, &staticMetaObject, 1, nullptr);
}

// SIGNAL 2
void ChessPlayer::startedThinking(int _t1)
{
    void *_a[] = { nullptr, const_cast<void*>(reinterpret_cast<const void*>(std::addressof(_t1))) };
    QMetaObject::activate(this, &staticMetaObject, 2, _a);
}

// SIGNAL 3
void ChessPlayer::stoppedThinking()
{
    QMetaObject::activate(this, &staticMetaObject, 3, nullptr);
}

// SIGNAL 4
void ChessPlayer::thinking(const MoveEvaluation & _t1)
{
    void *_a[] = { nullptr, const_cast<void*>(reinterpret_cast<const void*>(std::addressof(_t1))) };
    QMetaObject::activate(this, &staticMetaObject, 4, _a);
}

// SIGNAL 5
void ChessPlayer::moveMade(const Chess::Move & _t1)
{
    void *_a[] = { nullptr, const_cast<void*>(reinterpret_cast<const void*>(std::addressof(_t1))) };
    QMetaObject::activate(this, &staticMetaObject, 5, _a);
}

// SIGNAL 6
void ChessPlayer::resultClaim(const Chess::Result & _t1)
{
    void *_a[] = { nullptr, const_cast<void*>(reinterpret_cast<const void*>(std::addressof(_t1))) };
    QMetaObject::activate(this, &staticMetaObject, 6, _a);
}

// SIGNAL 7
void ChessPlayer::debugMessage(const QString & _t1)
{
    void *_a[] = { nullptr, const_cast<void*>(reinterpret_cast<const void*>(std::addressof(_t1))) };
    QMetaObject::activate(this, &staticMetaObject, 7, _a);
}

// SIGNAL 8
void ChessPlayer::nameChanged(const QString & _t1)
{
    void *_a[] = { nullptr, const_cast<void*>(reinterpret_cast<const void*>(std::addressof(_t1))) };
    QMetaObject::activate(this, &staticMetaObject, 8, _a);
}
QT_WARNING_POP
QT_END_MOC_NAMESPACE
