/****************************************************************************
** Meta object code from reading C++ file 'humanplayer.h'
**
** Created by: The Qt Meta Object Compiler version 67 (Qt 5.15.13)
**
** WARNING! All changes made in this file will be lost!
*****************************************************************************/

#include <memory>
#include "../../../projects/lib/src/humanplayer.h"
#include <QtCore/qbytearray.h>
#include <QtCore/qmetatype.h>
#if !defined(Q_MOC_OUTPUT_REVISION)
#error "The header file 'humanplayer.h' doesn't include <QObject>."
#elif Q_MOC_OUTPUT_REVISION != 67
#error "This file was generated using the moc from 5.15.13. It"
#error "cannot be used with the include files from this version of Qt."
#error "(The moc has changed too much.)"
#endif

QT_BEGIN_MOC_NAMESPACE
QT_WARNING_PUSH
QT_WARNING_DISABLE_DEPRECATED
struct qt_meta_stringdata_HumanPlayer_t {
    QByteArrayData data[8];
    char stringdata0[73];
};
#define QT_MOC_LITERAL(idx, ofs, len) \
    Q_STATIC_BYTE_ARRAY_DATA_HEADER_INITIALIZER_WITH_OFFSET(len, \
    qptrdiff(offsetof(qt_meta_stringdata_HumanPlayer_t, stringdata0) + ofs \
        - idx * sizeof(QByteArrayData)) \
    )
static const qt_meta_stringdata_HumanPlayer_t qt_meta_stringdata_HumanPlayer = {
    {
QT_MOC_LITERAL(0, 0, 11), // "HumanPlayer"
QT_MOC_LITERAL(1, 12, 6), // "wokeUp"
QT_MOC_LITERAL(2, 19, 0), // ""
QT_MOC_LITERAL(3, 20, 11), // "onHumanMove"
QT_MOC_LITERAL(4, 32, 18), // "Chess::GenericMove"
QT_MOC_LITERAL(5, 51, 4), // "move"
QT_MOC_LITERAL(6, 56, 11), // "Chess::Side"
QT_MOC_LITERAL(7, 68, 4) // "side"

    },
    "HumanPlayer\0wokeUp\0\0onHumanMove\0"
    "Chess::GenericMove\0move\0Chess::Side\0"
    "side"
};
#undef QT_MOC_LITERAL

static const uint qt_meta_data_HumanPlayer[] = {

 // content:
       8,       // revision
       0,       // classname
       0,    0, // classinfo
       2,   14, // methods
       0,    0, // properties
       0,    0, // enums/sets
       0,    0, // constructors
       0,       // flags
       1,       // signalCount

 // signals: name, argc, parameters, tag, flags
       1,    0,   24,    2, 0x06 /* Public */,

 // slots: name, argc, parameters, tag, flags
       3,    2,   25,    2, 0x0a /* Public */,

 // signals: parameters
    QMetaType::Void,

 // slots: parameters
    QMetaType::Void, 0x80000000 | 4, 0x80000000 | 6,    5,    7,

       0        // eod
};

void HumanPlayer::qt_static_metacall(QObject *_o, QMetaObject::Call _c, int _id, void **_a)
{
    if (_c == QMetaObject::InvokeMetaMethod) {
        auto *_t = static_cast<HumanPlayer *>(_o);
        (void)_t;
        switch (_id) {
        case 0: _t->wokeUp(); break;
        case 1: _t->onHumanMove((*reinterpret_cast< const Chess::GenericMove(*)>(_a[1])),(*reinterpret_cast< const Chess::Side(*)>(_a[2]))); break;
        default: ;
        }
    } else if (_c == QMetaObject::RegisterMethodArgumentMetaType) {
        switch (_id) {
        default: *reinterpret_cast<int*>(_a[0]) = -1; break;
        case 1:
            switch (*reinterpret_cast<int*>(_a[1])) {
            default: *reinterpret_cast<int*>(_a[0]) = -1; break;
            case 0:
                *reinterpret_cast<int*>(_a[0]) = qRegisterMetaType< Chess::GenericMove >(); break;
            case 1:
                *reinterpret_cast<int*>(_a[0]) = qRegisterMetaType< Chess::Side >(); break;
            }
            break;
        }
    } else if (_c == QMetaObject::IndexOfMethod) {
        int *result = reinterpret_cast<int *>(_a[0]);
        {
            using _t = void (HumanPlayer::*)();
            if (*reinterpret_cast<_t *>(_a[1]) == static_cast<_t>(&HumanPlayer::wokeUp)) {
                *result = 0;
                return;
            }
        }
    }
}

QT_INIT_METAOBJECT const QMetaObject HumanPlayer::staticMetaObject = { {
    QMetaObject::SuperData::link<ChessPlayer::staticMetaObject>(),
    qt_meta_stringdata_HumanPlayer.data,
    qt_meta_data_HumanPlayer,
    qt_static_metacall,
    nullptr,
    nullptr
} };


const QMetaObject *HumanPlayer::metaObject() const
{
    return QObject::d_ptr->metaObject ? QObject::d_ptr->dynamicMetaObject() : &staticMetaObject;
}

void *HumanPlayer::qt_metacast(const char *_clname)
{
    if (!_clname) return nullptr;
    if (!strcmp(_clname, qt_meta_stringdata_HumanPlayer.stringdata0))
        return static_cast<void*>(this);
    return ChessPlayer::qt_metacast(_clname);
}

int HumanPlayer::qt_metacall(QMetaObject::Call _c, int _id, void **_a)
{
    _id = ChessPlayer::qt_metacall(_c, _id, _a);
    if (_id < 0)
        return _id;
    if (_c == QMetaObject::InvokeMetaMethod) {
        if (_id < 2)
            qt_static_metacall(this, _c, _id, _a);
        _id -= 2;
    } else if (_c == QMetaObject::RegisterMethodArgumentMetaType) {
        if (_id < 2)
            qt_static_metacall(this, _c, _id, _a);
        _id -= 2;
    }
    return _id;
}

// SIGNAL 0
void HumanPlayer::wokeUp()
{
    QMetaObject::activate(this, &staticMetaObject, 0, nullptr);
}
QT_WARNING_POP
QT_END_MOC_NAMESPACE
