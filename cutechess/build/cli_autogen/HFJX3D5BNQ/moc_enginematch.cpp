/****************************************************************************
** Meta object code from reading C++ file 'enginematch.h'
**
** Created by: The Qt Meta Object Compiler version 67 (Qt 5.15.13)
**
** WARNING! All changes made in this file will be lost!
*****************************************************************************/

#include <memory>
#include "../../../projects/cli/src/enginematch.h"
#include <QtCore/qbytearray.h>
#include <QtCore/qmetatype.h>
#if !defined(Q_MOC_OUTPUT_REVISION)
#error "The header file 'enginematch.h' doesn't include <QObject>."
#elif Q_MOC_OUTPUT_REVISION != 67
#error "This file was generated using the moc from 5.15.13. It"
#error "cannot be used with the include files from this version of Qt."
#error "(The moc has changed too much.)"
#endif

QT_BEGIN_MOC_NAMESPACE
QT_WARNING_PUSH
QT_WARNING_DISABLE_DEPRECATED
struct qt_meta_stringdata_EngineMatch_t {
    QByteArrayData data[11];
    char stringdata0[105];
};
#define QT_MOC_LITERAL(idx, ofs, len) \
    Q_STATIC_BYTE_ARRAY_DATA_HEADER_INITIALIZER_WITH_OFFSET(len, \
    qptrdiff(offsetof(qt_meta_stringdata_EngineMatch_t, stringdata0) + ofs \
        - idx * sizeof(QByteArrayData)) \
    )
static const qt_meta_stringdata_EngineMatch_t qt_meta_stringdata_EngineMatch = {
    {
QT_MOC_LITERAL(0, 0, 11), // "EngineMatch"
QT_MOC_LITERAL(1, 12, 8), // "finished"
QT_MOC_LITERAL(2, 21, 0), // ""
QT_MOC_LITERAL(3, 22, 13), // "onGameStarted"
QT_MOC_LITERAL(4, 36, 10), // "ChessGame*"
QT_MOC_LITERAL(5, 47, 4), // "game"
QT_MOC_LITERAL(6, 52, 6), // "number"
QT_MOC_LITERAL(7, 59, 14), // "onGameFinished"
QT_MOC_LITERAL(8, 74, 20), // "onTournamentFinished"
QT_MOC_LITERAL(9, 95, 5), // "print"
QT_MOC_LITERAL(10, 101, 3) // "msg"

    },
    "EngineMatch\0finished\0\0onGameStarted\0"
    "ChessGame*\0game\0number\0onGameFinished\0"
    "onTournamentFinished\0print\0msg"
};
#undef QT_MOC_LITERAL

static const uint qt_meta_data_EngineMatch[] = {

 // content:
       8,       // revision
       0,       // classname
       0,    0, // classinfo
       5,   14, // methods
       0,    0, // properties
       0,    0, // enums/sets
       0,    0, // constructors
       0,       // flags
       1,       // signalCount

 // signals: name, argc, parameters, tag, flags
       1,    0,   39,    2, 0x06 /* Public */,

 // slots: name, argc, parameters, tag, flags
       3,    2,   40,    2, 0x08 /* Private */,
       7,    2,   45,    2, 0x08 /* Private */,
       8,    0,   50,    2, 0x08 /* Private */,
       9,    1,   51,    2, 0x08 /* Private */,

 // signals: parameters
    QMetaType::Void,

 // slots: parameters
    QMetaType::Void, 0x80000000 | 4, QMetaType::Int,    5,    6,
    QMetaType::Void, 0x80000000 | 4, QMetaType::Int,    5,    6,
    QMetaType::Void,
    QMetaType::Void, QMetaType::QString,   10,

       0        // eod
};

void EngineMatch::qt_static_metacall(QObject *_o, QMetaObject::Call _c, int _id, void **_a)
{
    if (_c == QMetaObject::InvokeMetaMethod) {
        auto *_t = static_cast<EngineMatch *>(_o);
        (void)_t;
        switch (_id) {
        case 0: _t->finished(); break;
        case 1: _t->onGameStarted((*reinterpret_cast< ChessGame*(*)>(_a[1])),(*reinterpret_cast< int(*)>(_a[2]))); break;
        case 2: _t->onGameFinished((*reinterpret_cast< ChessGame*(*)>(_a[1])),(*reinterpret_cast< int(*)>(_a[2]))); break;
        case 3: _t->onTournamentFinished(); break;
        case 4: _t->print((*reinterpret_cast< const QString(*)>(_a[1]))); break;
        default: ;
        }
    } else if (_c == QMetaObject::IndexOfMethod) {
        int *result = reinterpret_cast<int *>(_a[0]);
        {
            using _t = void (EngineMatch::*)();
            if (*reinterpret_cast<_t *>(_a[1]) == static_cast<_t>(&EngineMatch::finished)) {
                *result = 0;
                return;
            }
        }
    }
}

QT_INIT_METAOBJECT const QMetaObject EngineMatch::staticMetaObject = { {
    QMetaObject::SuperData::link<QObject::staticMetaObject>(),
    qt_meta_stringdata_EngineMatch.data,
    qt_meta_data_EngineMatch,
    qt_static_metacall,
    nullptr,
    nullptr
} };


const QMetaObject *EngineMatch::metaObject() const
{
    return QObject::d_ptr->metaObject ? QObject::d_ptr->dynamicMetaObject() : &staticMetaObject;
}

void *EngineMatch::qt_metacast(const char *_clname)
{
    if (!_clname) return nullptr;
    if (!strcmp(_clname, qt_meta_stringdata_EngineMatch.stringdata0))
        return static_cast<void*>(this);
    return QObject::qt_metacast(_clname);
}

int EngineMatch::qt_metacall(QMetaObject::Call _c, int _id, void **_a)
{
    _id = QObject::qt_metacall(_c, _id, _a);
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

// SIGNAL 0
void EngineMatch::finished()
{
    QMetaObject::activate(this, &staticMetaObject, 0, nullptr);
}
QT_WARNING_POP
QT_END_MOC_NAMESPACE
