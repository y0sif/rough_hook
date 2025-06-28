/****************************************************************************
** Meta object code from reading C++ file 'chessclock.h'
**
** Created by: The Qt Meta Object Compiler version 67 (Qt 5.15.13)
**
** WARNING! All changes made in this file will be lost!
*****************************************************************************/

#include <memory>
#include "../../../projects/gui/src/chessclock.h"
#include <QtCore/qbytearray.h>
#include <QtCore/qmetatype.h>
#if !defined(Q_MOC_OUTPUT_REVISION)
#error "The header file 'chessclock.h' doesn't include <QObject>."
#elif Q_MOC_OUTPUT_REVISION != 67
#error "This file was generated using the moc from 5.15.13. It"
#error "cannot be used with the include files from this version of Qt."
#error "(The moc has changed too much.)"
#endif

QT_BEGIN_MOC_NAMESPACE
QT_WARNING_PUSH
QT_WARNING_DISABLE_DEPRECATED
struct qt_meta_stringdata_ChessClock_t {
    QByteArrayData data[10];
    char stringdata0[85];
};
#define QT_MOC_LITERAL(idx, ofs, len) \
    Q_STATIC_BYTE_ARRAY_DATA_HEADER_INITIALIZER_WITH_OFFSET(len, \
    qptrdiff(offsetof(qt_meta_stringdata_ChessClock_t, stringdata0) + ofs \
        - idx * sizeof(QByteArrayData)) \
    )
static const qt_meta_stringdata_ChessClock_t qt_meta_stringdata_ChessClock = {
    {
QT_MOC_LITERAL(0, 0, 10), // "ChessClock"
QT_MOC_LITERAL(1, 11, 13), // "setPlayerName"
QT_MOC_LITERAL(2, 25, 0), // ""
QT_MOC_LITERAL(3, 26, 4), // "name"
QT_MOC_LITERAL(4, 31, 15), // "setInfiniteTime"
QT_MOC_LITERAL(5, 47, 8), // "infinite"
QT_MOC_LITERAL(6, 56, 7), // "setTime"
QT_MOC_LITERAL(7, 64, 9), // "totalTime"
QT_MOC_LITERAL(8, 74, 5), // "start"
QT_MOC_LITERAL(9, 80, 4) // "stop"

    },
    "ChessClock\0setPlayerName\0\0name\0"
    "setInfiniteTime\0infinite\0setTime\0"
    "totalTime\0start\0stop"
};
#undef QT_MOC_LITERAL

static const uint qt_meta_data_ChessClock[] = {

 // content:
       8,       // revision
       0,       // classname
       0,    0, // classinfo
       5,   14, // methods
       0,    0, // properties
       0,    0, // enums/sets
       0,    0, // constructors
       0,       // flags
       0,       // signalCount

 // slots: name, argc, parameters, tag, flags
       1,    1,   39,    2, 0x0a /* Public */,
       4,    1,   42,    2, 0x0a /* Public */,
       6,    1,   45,    2, 0x0a /* Public */,
       8,    1,   48,    2, 0x0a /* Public */,
       9,    0,   51,    2, 0x0a /* Public */,

 // slots: parameters
    QMetaType::Void, QMetaType::QString,    3,
    QMetaType::Void, QMetaType::Bool,    5,
    QMetaType::Void, QMetaType::Int,    7,
    QMetaType::Void, QMetaType::Int,    7,
    QMetaType::Void,

       0        // eod
};

void ChessClock::qt_static_metacall(QObject *_o, QMetaObject::Call _c, int _id, void **_a)
{
    if (_c == QMetaObject::InvokeMetaMethod) {
        auto *_t = static_cast<ChessClock *>(_o);
        (void)_t;
        switch (_id) {
        case 0: _t->setPlayerName((*reinterpret_cast< const QString(*)>(_a[1]))); break;
        case 1: _t->setInfiniteTime((*reinterpret_cast< bool(*)>(_a[1]))); break;
        case 2: _t->setTime((*reinterpret_cast< int(*)>(_a[1]))); break;
        case 3: _t->start((*reinterpret_cast< int(*)>(_a[1]))); break;
        case 4: _t->stop(); break;
        default: ;
        }
    }
}

QT_INIT_METAOBJECT const QMetaObject ChessClock::staticMetaObject = { {
    QMetaObject::SuperData::link<QWidget::staticMetaObject>(),
    qt_meta_stringdata_ChessClock.data,
    qt_meta_data_ChessClock,
    qt_static_metacall,
    nullptr,
    nullptr
} };


const QMetaObject *ChessClock::metaObject() const
{
    return QObject::d_ptr->metaObject ? QObject::d_ptr->dynamicMetaObject() : &staticMetaObject;
}

void *ChessClock::qt_metacast(const char *_clname)
{
    if (!_clname) return nullptr;
    if (!strcmp(_clname, qt_meta_stringdata_ChessClock.stringdata0))
        return static_cast<void*>(this);
    return QWidget::qt_metacast(_clname);
}

int ChessClock::qt_metacall(QMetaObject::Call _c, int _id, void **_a)
{
    _id = QWidget::qt_metacall(_c, _id, _a);
    if (_id < 0)
        return _id;
    if (_c == QMetaObject::InvokeMetaMethod) {
        if (_id < 5)
            qt_static_metacall(this, _c, _id, _a);
        _id -= 5;
    } else if (_c == QMetaObject::RegisterMethodArgumentMetaType) {
        if (_id < 5)
            *reinterpret_cast<int*>(_a[0]) = -1;
        _id -= 5;
    }
    return _id;
}
QT_WARNING_POP
QT_END_MOC_NAMESPACE
